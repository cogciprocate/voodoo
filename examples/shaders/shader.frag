#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(binding = 1) uniform sampler2D texSampler;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;

layout(location = 0) out vec4 outColor;

void main() {
	// DEBUG COORDS:
    // outColor = vec4(fragTexCoord, 0.0, 1.0);
    // outColor = texture(texSampler, fragTexCoord * 2.0);
    // outColor = vec4(fragColor * texture(texSampler, fragTexCoord).rgb, 1.0);
    // outColor = texture(texSampler, fragTexCoord);
    // outColor = vec4(ceil(texture(texSampler, fragTexCoord)).rgb * fragColor, 1.0);

    if (texture(texSampler, fragTexCoord).rgb == vec3(0.0, 0.0, 0.0)) {
    	discard;
    }
    outColor = vec4(ceil(texture(texSampler, fragTexCoord)).rgb * fragColor, 0.1);
}