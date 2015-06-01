/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;

pub type ShHandle = *mut c_void;
pub type ShaderVariable = *mut c_void;
pub type Uniform = *mut c_void;
pub type Attribute = *mut c_void;
pub type InterfaceBlockField = *mut c_void;
pub type Varying = *mut c_void;
pub type InterfaceBlock = *mut c_void;

pub type GLenum = c_uint;

pub type enum_ShArrayIndexClampingStrategy = c_uint;
pub static SH_CLAMP_WITH_CLAMP_INTRINSIC: u32 = 1_u32;
pub static SH_CLAMP_WITH_USER_DEFINED_INT_CLAMP_FUNCTION: u32 = 2_u32;
pub type ShArrayIndexClampingStrategy = enum_ShArrayIndexClampingStrategy;

pub type enum_ShShaderSpec = c_uint;
pub static SH_GLES2_SPEC : u32 = 0x8B40;
pub static SH_WEBGL_SPEC : u32 = 0x8B41;
pub static SH_GLES3_SPEC : u32 = 0x8B86;
pub static SH_WEBGL2_SPEC : u32 = 0x8B87;
pub static SH_CSS_SHADERS_SPEC : u32 = 0x8B4;
pub type ShShaderSpec = enum_ShShaderSpec;

pub type enum_ShShaderOutput = c_uint;
pub static SH_ESSL_OUTPUT : u32 = 0x8B45;
pub static SH_GLSL_OUTPUT : u32 = 0x8B46;
pub static SH_GLSL_COMPATIBILITY_OUTPUT : u32 = 0x8B46;
pub static SH_GLSL_CORE_OUTPUT : u32 = 0x8B47;
pub static SH_GLSL_130_OUTPUT : u32 = 0x8B47;
pub static SH_GLSL_410_CORE_OUTPUT : u32 = 0x8B84;
pub static SH_GLSL_420_CORE_OUTPUT : u32 = 0x8B85;
pub static SH_HLSL_OUTPUT : u32  = 0x8B48;
pub static SH_HLSL9_OUTPUT : u32 = 0x8B48;
pub static SH_HLSL11_OUTPUT : u32 = 0x8B4;
pub type ShShaderOutput = enum_ShShaderOutput;

#[repr(C)]
pub struct struct__ShBuiltInResources {
    pub MaxVertexAttribs: c_int,
    pub MaxVertexUniformVectors: c_int,
    pub MaxVaryingVectors: c_int,
    pub MaxVertexTextureImageUnits: c_int,
    pub MaxCombinedTextureImageUnits: c_int,
    pub MaxTextureImageUnits: c_int,
    pub MaxFragmentUniformVectors: c_int,
    pub MaxDrawBuffers: c_int,

    pub OES_standard_derivatives: c_int,
    pub OES_EGL_image_external: c_int,
    pub ARB_texture_rectangle: c_int,
    pub EXT_draw_buffers: c_int,
    pub EXT_frag_depth: c_int,
    pub EXT_shader_texture_lod: c_int,
    pub WEBGL_debug_shader_precision: c_int,
    pub EXT_shader_framebuffer_fetch: c_int,
    pub NV_shader_framebuffer_fetch: c_int,
    pub ARM_shader_framebuffer_fetch: c_int,

    pub NV_draw_buffers: c_int,

    pub FragmentPrecisionHigh: c_int,

    pub MaxVertexOutputVectors: c_int,
    pub MaxFragmentInputVectors: c_int,
    pub MinProgramTexelOffset: c_int,
    pub MaxProgramTexelOffset: c_int,

    pub HashFunction: Option<extern fn(*mut c_char, size_t)>,

    pub ArrayIndexClampingStrategy: ShArrayIndexClampingStrategy,

    pub MaxExpressionComplexity: c_int,

    pub MaxCallStackDepth: c_int,
}
pub type ShBuiltInResources = struct__ShBuiltInResources;

#[repr(C)]
pub struct struct__ShVariableInfo {
    pub variable_type : GLenum,
    pub size: c_int,
}
pub type ShVariableInfo = struct__ShVariableInfo;

#[link(name = "angle")]
extern {
pub fn Initialize() -> bool;
pub fn ShFinalize();
pub fn ShInitBuiltInResources(resources: *mut ShBuiltInResources);
pub fn ShGetBuiltInResourcesStringC(handle: ShHandle) -> *const c_char;
pub fn ShConstructCompiler(shaderType: GLenum, spec: ShShaderSpec, output: ShShaderOutput, resources: *mut ShBuiltInResources);
pub fn ShDestruct(handle: ShHandle);
pub fn ShCompile(handle: ShHandle, shaderStrings: *const *const c_char, numStrings: size_t, compileOptions: c_int);
pub fn ShGetShaderVersion(handle: ShHandle) -> c_int;
pub fn ShGetInfoLogC(handle: ShHandle) -> *mut c_char;// 
pub fn ShGetObjectCodeC(handle: ShHandle) -> *mut c_char;// 
pub fn ShGetNameHashingEntry(handle: ShHandle, index: c_int, name: *mut c_char, hashedName: *mut c_char); // 

pub fn ShGetUniformsC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetVaryingsC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetGetAttributesC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetGetGetOutputVariablesC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetGetGetInterfaceBlocksC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShCheckVaraiblesWithinPackingLimits(maxVectors: c_int, varInfoArray: *const ShVariableInfo, varInfoArraySize: size_t);

pub fn ShaderVariableGetType(shader_variable: ShaderVariable) -> GLenum;
pub fn ShaderVariableGetPrecision(shader_variable: ShaderVariable) -> GLenum;
pub fn ShaderVariableGetName(shader_variable: ShaderVariable) -> *const c_char;
pub fn ShaderVariableGetMappedName(shader_variable: ShaderVariable) -> *const c_char;
pub fn ShaderVariableGetFields(shader_variable: ShaderVariable) -> *const ShaderVariable;
pub fn ShaderVariableGetStructName(shader_variable: ShaderVariable) -> *const c_char;

}

