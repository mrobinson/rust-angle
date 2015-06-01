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

#[link(name = "angle")]
extern {
pub fn ShInitialize() -> bool;
pub fn ShFinalize();
pub fn ShInitBuiltInResources(resources: *mut ShBuiltInResources);
pub fn ShGetBuiltInResourcesStringCString(handle: ShHandle) -> *const c_char;// 
pub fn ShConstructCompiler(shaderType: GLenum, spec: ShShaderSpec, output: ShShaderOutput, resources: *mut ShBuiltInResources);
pub fn ShDestruct(handle: ShHandle);
pub fn ShCompile(handle: ShHandle, shaderStrings: *const *const c_char, numStrings: size_t, compileOptions: c_int);
pub fn ShGetShaderVersion(handle: ShHandle) -> c_int;
pub fn ShGetInfoLogCString(handle: ShHandle) -> *mut c_char;// 
pub fn ShGetObjectCodeCString(handle: ShHandle) -> *mut c_char;// 
pub fn ShGetNameHashingEntry(handle: ShHandle, index: c_int, name: *mut c_char, hashedName: *mut c_char); // 

pub fn ShGetUniformsC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetVaryingsC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetGetAttributesC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetGetGetOutputVariablesC(handle: ShHandle) -> *const ShaderVariable;
pub fn ShGetGetGetInterfaceBlocksC(handle: ShHandle) -> *const ShaderVariable;

//COMPILER_EXPORT const std::vector<sh::Uniform> *ShGetUniforms(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::Varying> *ShGetVaryings(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::Attribute> *ShGetAttributes(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::Attribute> *ShGetOutputVariables(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::InterfaceBlock> *ShGetInterfaceBlocks(const ShHandle handle);

pub fn ShaderVariableGetType(shader_variable: ShaderVariable) -> GLenum;
pub fn ShaderVariableGetPrecision(shader_variable: ShaderVariable) -> GLenum;
pub fn ShaderVariableGetName(shader_variable: ShaderVariable) -> *const c_char;
pub fn ShaderVariableGetMappedName(shader_variable: ShaderVariable) -> *const c_char;
pub fn ShaderVariableGetFields(shader_variable: ShaderVariable) -> *const ShaderVariable;
pub fn ShaderVariableGetStructName(shader_variable: ShaderVariable) -> *const c_char;

}

//COMPILER_EXPORT ShHandle ShConstructCompiler(
//    sh::GLenum type,
//    ShShaderSpec spec,
//    ShShaderOutput output,
//    const ShBuiltInResources *resources);
//COMPILER_EXPORT void ShDestruct(ShHandle handle);
//
//
//COMPILER_EXPORT bool ShCompile(
//    const ShHandle handle,
//    const char * const shaderStrings[],
//    size_t numStrings,
//    int compileOptions);
//
//COMPILER_EXPORT int ShGetShaderVersion(const ShHandle handle);
//
//COMPILER_EXPORT ShShaderOutput ShGetShaderOutputType(
//    const ShHandle handle);
//
//COMPILER_EXPORT const std::string &ShGetInfoLog(const ShHandle handle);
//
//COMPILER_EXPORT const std::string &ShGetObjectCode(const ShHandle handle);
//
//COMPILER_EXPORT const std::map<std::string, std::string> *ShGetNameHashingMap(
//    const ShHandle handle);
//
//COMPILER_EXPORT const std::vector<sh::Uniform> *ShGetUniforms(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::Varying> *ShGetVaryings(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::Attribute> *ShGetAttributes(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::Attribute> *ShGetOutputVariables(const ShHandle handle);
//COMPILER_EXPORT const std::vector<sh::InterfaceBlock> *ShGetInterfaceBlocks(const ShHandle handle);
//
//COMPILER_EXPORT bool ShCheckVariablesWithinPackingLimits(
//    int maxVectors,
//    ShVariableInfo *varInfoArray,
//    size_t varInfoArraySize);
//
//COMPILER_EXPORT bool ShGetInterfaceBlockRegister(const ShHandle handle,
//                                                 const std::string &interfaceBlockName,
//                                                 unsigned int *indexOut);
//
//COMPILER_EXPORT bool ShGetUniformRegister(const ShHandle handle,
//                                          const std::string &uniformName,
//                                          unsigned int *indexOut);
//
//
//ShCheckVariablesWithinPackingLimits(maxVaryingVectors, variables, numVaryings);
//ShCompile(compiler, shaderSourceStrings, 1, SH_OBJECT_CODE | SH_VARIABLES | extraCompileOptions);
//ShConstructCompiler(SH_FRAGMENT_SHADER, m_shaderSpec, m_shaderOutput, &m_resources);
//ShConstructCompiler(SH_VERTEX_SHADER, m_shaderSpec, m_shaderOutput, &m_resources);
//ShDestruct(m_fragmentCompiler);
//ShDestruct(m_vertexCompiler);
//ShGetInfo(compiler, shaderInfo, &value);
//ShGetInfoLog(compiler, logBuffer.get());
//ShGetObjectCode(compiler, translationBuffer.get());
//ShGetVariableInfo(compiler, symbolType, i, &nameLength, &symbol.size, &symbol.dataType, &precision, &staticUse, nameBuffer.data(), mappedNameBuffer.data());
//ShGetVariableInfo(compiler, symbolType, i, &nameLength, &symbol.size, &symbol.dataType, &precision, &staticUse, nameBuffer.data(), mappedNameBuffer.data());
//ShGetVariableInfo(compiler, symbolType, i, &nameLength, &symbol.size, &symbol.dataType, &precision, &staticUse, nameBuffer.data(), mappedNameBuffer.data());
//ShInitBuiltInResources(&ANGLEResources);
//ShInitBuiltInResources(&ANGLEResources);
//ShInitBuiltInResources(&ANGLEResources);
//ShInitBuiltInResources(&ANGLEResources);
//ShInitialize();

