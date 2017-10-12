use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader};
use std::ops::Deref;
use std::slice;
use libc::c_char;
use ::{VooResult, PRINT};

/// An owned or borrowed C string representable as a pointer.
#[derive(Debug, Clone)]
pub enum CharStr<'cp> {
    CString(CString),
    CStr(&'cp CStr),
}

impl<'cp> Deref for CharStr<'cp> {
    type Target = CStr;

    fn deref(&self) -> &CStr {
        match *self {
            CharStr::CString(ref cs) => cs.as_c_str(),
            CharStr::CStr(ref cs) => cs,
        }
    }
}

impl<'cp, 's> From<&'s CStr> for CharStr<'cp> where 's: 'cp {
    fn from(s: &'s CStr) -> CharStr<'cp> {
        CharStr::CStr(s)
    }
}

impl<'cp, 's> From<&'s [u8]> for CharStr<'cp> where 's: 'cp {
    fn from(s: &'s [u8]) -> CharStr<'cp> {
        CharStr::CStr(
            CStr::from_bytes_with_nul(s)
                .expect(&format!("unable to convert '{:?}' to a valid C string", s))
        )
    }
}

impl<'cp> From<CString> for CharStr<'cp> {
    fn from(s: CString) -> CharStr<'cp> {
        CharStr::CString(s)
    }
}

impl<'cp, 's> From<&'s str> for CharStr<'cp> where 's: 'cp {
    fn from(s: &'s str) -> CharStr<'cp> {
        CharStr::CString(
            CString::new(s)
                .expect(&format!("unable to convert '{:?}' to a valid C string", s))
        )
    }
}

impl<'cp> From<String> for CharStr<'cp> {
    fn from(s: String) -> CharStr<'cp> {
        CharStr::CString(
            CString::new(s)
                .expect(&format!("unable to convert to a valid C string"))
        )
    }
}


/// Either a borrowed list of borrowed char pointers, an owned list of
/// borrowed char pointers, or owned lists of both `CString`s and pointers to
/// their internal arrays.
#[derive(Debug, Clone)]
pub enum CharStrs<'cs> {
    Ptr { ptr: *const *const c_char, len: usize },
    RefPtr { ptrs: &'cs [*const c_char] },
    OwnedPtr { ptrs: Vec<*const c_char> },
    OwnedOwned { strings: Vec<CString>, ptrs: Vec<*const c_char> },
}

impl<'cs> CharStrs<'cs> {
    pub fn len(&self) -> usize {
        match *self {
            CharStrs::Ptr { len, .. } => len,
            CharStrs::RefPtr { ref ptrs } => ptrs.len(),
            CharStrs::OwnedPtr { ref ptrs } => ptrs.len(),
            CharStrs::OwnedOwned {ref ptrs, .. } => ptrs.len(),
        }
    }

    pub fn as_ptr(&self) -> *const *const c_char {
        match *self {
            CharStrs::Ptr { ptr, .. } => ptr,
            CharStrs::RefPtr { ref ptrs } => ptrs.as_ptr(),
            CharStrs::OwnedPtr { ref ptrs } => ptrs.as_ptr(),
            CharStrs::OwnedOwned {ref ptrs, .. } => ptrs.as_ptr(),
        }
    }

    pub fn as_ptr_slice(&self) -> &'cs [*const c_char] {
        // match *self {
        //     CharStrs::Ptr { ptr, len } => unsafe { slice::from_raw_parts(ptr, len) },
        //     CharStrs::RefPtr { ref ptrs } => ptrs,
        //     CharStrs::OwnedPtr { ref ptrs } => ptrs.as_slice(),
        //     CharStrs::OwnedOwned {ref ptrs, .. } => ptrs.as_slice(),
        // }
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

// impl <'cs, 'p> From<(*const *const c_char, usize)> for CharStrs<'cs> where 'p: 'cs {
//     fn from(ptrs: &'p [*const c_char]) -> CharStrs<'cs> {
//         CharStrs::RefPtr { ptrs }
//     }
// }

impl <'cs, 'p> From<&'p [*const c_char]> for CharStrs<'cs> where 'p: 'cs {
    fn from(ptrs: &'p [*const c_char]) -> CharStrs<'cs> {
        CharStrs::RefPtr { ptrs }
    }
}

impl <'cs, 'p, 'q> From<&'p [&'q [u8]]> for CharStrs<'cs> where 'q: 'p, 'p: 'cs, {
    fn from(slices: &'p [&'q [u8]]) -> CharStrs<'cs> {
        // The pointers to the `CStr` will be == pointers to the byte slices.
        // Running through a `CStr` verifies the contents.
        let ptrs = slices.iter().map(|slice| {
            let ptr = CStr::from_bytes_with_nul(slice)
                .expect(&format!("unable to convert '{:?}' to a valid C string", slice))
                .as_ptr();
            debug_assert_eq!(ptr, slice.as_ptr() as *const i8);
            ptr
        }).collect();
        CharStrs::OwnedPtr { ptrs }
    }
}

impl <'cs, 'p, 'q> From<&'p [&'q CStr]> for CharStrs<'cs> where 'q: 'p, 'p: 'cs, {
    fn from(slices: &'p [&'q CStr]) -> CharStrs<'cs> {
        let ptrs = slices.iter().map(|cstr| cstr.as_ptr()).collect();
        CharStrs::OwnedPtr { ptrs }
    }
}

impl <'cs, 'p, 'q> From<&'p [&'q str]> for CharStrs<'cs> where 'q: 'p, 'p: 'cs, {
    fn from(slices: &'p [&'q str]) -> CharStrs<'cs> {
        let strings: Vec<CString> = slices.iter().map(|&s| {
            CString::new(s)
                .expect(&format!("unable to convert '{:?}' to a valid C string", s))
        }).collect();
        let ptrs = strings.iter().map(|cstring| cstring.as_ptr()).collect();
        CharStrs::OwnedOwned { strings, ptrs }
    }
}


pub fn read_spir_v_file<P: AsRef<Path>>(file: P) -> VooResult<Vec<u32>> {
    let mut contents = read_file(file)?;
    assert!(contents.len() % 4 == 0);
    assert!(mem::size_of_val(&contents[0]) == 1);
    // TODO: Add some sort of basic verification that the file is actually
    // spir-v.
    unsafe {
        let ptr = contents.as_mut_ptr() as *mut u32;
        let new_len = contents.len() / 4;
        mem::forget(contents);
        let code = Vec::from_raw_parts(ptr, new_len, new_len);
        Ok(code)
    }
}

/// Reads a file into a byte Vec.
pub fn read_file<P: AsRef<Path>>(file: P) -> VooResult<Vec<u8>> {
    let file_name = file.as_ref().display().to_string();
    let f = File::open(file).expect("shader file not found");
    let file_bytes = f.metadata().unwrap().len() as usize;
    let mut contents = Vec::<u8>::with_capacity(file_bytes);
    let mut reader = BufReader::new(f);
    match reader.read_to_end(&mut contents) {
        Ok(bytes) => {
            assert_eq!(bytes, file_bytes);
            if PRINT { println!("Read {} bytes from {}", bytes, &file_name); }
        },
        Err(e) => panic!("{}", e),
    }
    Ok(contents)
}

pub fn file_reader<P: AsRef<Path>>(file: P) -> VooResult<BufReader<File>> {
    // let file_name = file.as_ref().display().to_string();
    let f = File::open(file).expect("file not found");
    // let file_bytes = f.metadata().unwrap().len() as usize;
    // let mut contents = Vec::<u8>::with_capacity(file_bytes);
    Ok(BufReader::new(f))
}


/// Returns a column-major perspective matrix.
pub fn persp_matrix(width: u32, height: u32, fov_zoom: f32) -> [[f32; 4]; 4] {
    let zfar = 1024.0;
    let znear = 0.1;

    // let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;
    let fov: f32 = 3.141592 / fov_zoom;
    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}

/// Returns a column-major view matrix.
pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}





        // ubo.model = glm::rotate(glm::mat4(1.0f), time * glm::radians(90.0f),
        //     glm::vec3(0.0f, 0.0f, 1.0f));

        // let axis = Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0f32));
        // debug_assert_eq!(Unit::new_normalize(Vector3::z()), axis);
        // let angle = (time * (90.0f32)).to_radians();
        // let rotation = Rotation3::from_axis_angle(&axis, angle);
        // let rotation_matrix = Matrix4::<f32>::from_scaled_axis(rotation.scaled_axis());
        // let rotation_matrix_2 = Matrix4::<f32>::new_rotation(rotation.scaled_axis());
        // let rotation_matrix_3 = rotation.to_homogeneous();
        // let identity_matrix = Matrix4::<f32>::identity();
        // debug_assert_eq!(rotation_matrix, rotation_matrix_2);
        // debug_assert_eq!(rotation_matrix, rotation_matrix_3);
        // debug_assert_eq!(rotation_matrix * identity_matrix, rotation_matrix);


        // let model_rotation_angle = (time * (90.0f32)).to_radians();
        // // let model_rotation_angle = ((90.0f32)).to_radians();
        // let model_rotation_axis = Vector3::z();
        // let model_rotation_vector = model_rotation_angle * model_rotation_axis;
        // let model_translation_vector = nalgebra::zero::<Vector3<f32>>();
        // let model_isometry = Isometry3::new(model_translation_vector,
        //     model_rotation_vector);
        // let model_transformation_matrix: Matrix4<_> = model_isometry.to_homogeneous();

        // let eye = Point3::new(2.0, 2.0, 2.0f32);
        // let target = Point3::new(0.0, 0.0, 0.0);
        // let up = Vector3::y();
        // let view_isometry = Isometry3::look_at_rh(&eye, &target, &up);
        // let view_transformation_matrix = view_isometry.to_homogeneous();

        // let extent = self.swapchain.as_ref().unwrap().extent().clone();
        // debug_assert_eq!(PI / 4.0, 45.0f32.to_radians());
        // let projection_perspective = Perspective3::new(PI / 4.,
        //     extent.width as f32 / extent.height as f32, 0.1, 10.0f32);
        // let mut projection_matrix = projection_perspective.to_homogeneous();
        // // Flip sign because y is inverted in Vulkan.
        // projection_matrix[(1, 1)] *= -1.0;

        // let ubo = vkc::UniformBufferObject {
        //     model: model_transformation_matrix.into(),
        //     view: view_transformation_matrix.into(),
        //     proj: projection_matrix.into(),
        // };
