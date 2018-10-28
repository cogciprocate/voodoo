use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader};
use std::ops::Deref;
use std::slice;
use libc::c_char;
use ::{VdResult, PRINT};

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
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}


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


/// Reads a SPIR-V file into a word Vec.
pub fn read_spir_v_file<P: AsRef<Path>>(file: P) -> VdResult<Vec<u32>> {
    let mut contents = read_file(file)?;
    assert!(contents.len() % 4 == 0);
    assert!(mem::size_of_val(&contents[0]) == 1);
    // TODO: Add some sort of basic verification that the file is actually
    // spir-v.
    unsafe {
        let ptr = contents.as_ptr() as *const u32;
        let new_len = contents.len() / 4;
        let slice: &[u32] = slice::from_raw_parts(ptr, new_len);
        let vec = slice.to_vec();
        Ok(vec)
    }
}

/// Reads a file into a byte Vec.
pub fn read_file<P: AsRef<Path>>(file: P) -> VdResult<Vec<u8>> {
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

pub fn file_reader<P: AsRef<Path>>(file: P) -> VdResult<BufReader<File>> {
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


