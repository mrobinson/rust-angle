//
// Copyright (c) 2015 The ANGLE Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//

// PbufferSurfaceGLX.cpp: GLX implementation of egl::Surface for PBuffers

#include "libANGLE/renderer/gl/glx/PbufferSurfaceGLX.h"

#include "common/debug.h"
#include "libANGLE/renderer/gl/glx/FunctionsGLX.h"

namespace rx
{

PbufferSurfaceGLX::PbufferSurfaceGLX(EGLint width, EGLint height, bool largest, const FunctionsGLX &glx,
                                     Display *display, GLXContext context, GLXFBConfig fbConfig)
    : SurfaceGL(),
      mWidth(width),
      mHeight(height),
      mLargest(largest),
      mGLX(glx),
      mDisplay(display),
      mContext(context),
      mFBConfig(fbConfig),
      mPbuffer(0)
{
}

PbufferSurfaceGLX::~PbufferSurfaceGLX()
{
    if (mPbuffer)
    {
        mGLX.destroyPbuffer(mDisplay, mPbuffer);
    }
}

egl::Error PbufferSurfaceGLX::initialize()
{
    const int attribs[] =
    {
        GLX_PBUFFER_WIDTH, static_cast<int>(mWidth),
        GLX_PBUFFER_HEIGHT, static_cast<int>(mHeight),
        GLX_LARGEST_PBUFFER, mLargest,
        None
    };

    mPbuffer = mGLX.createPbuffer(mDisplay, mFBConfig, attribs);
    if (!mPbuffer)
    {
        return egl::Error(EGL_BAD_ALLOC, "Failed to create a native GLX pbuffer.");
    }

    if (mLargest)
    {
        mGLX.queryDrawable(mDisplay, mPbuffer, GLX_WIDTH, &mWidth);
        mGLX.queryDrawable(mDisplay, mPbuffer, GLX_HEIGHT, &mHeight);
    }

    return egl::Error(EGL_SUCCESS);
}

egl::Error PbufferSurfaceGLX::makeCurrent()
{
    if (mGLX.makeCurrent(mDisplay, mPbuffer, mContext) != True)
    {
        return egl::Error(EGL_BAD_DISPLAY);
    }
    return egl::Error(EGL_SUCCESS);
}

egl::Error PbufferSurfaceGLX::swap()
{
    return egl::Error(EGL_SUCCESS);
}

egl::Error PbufferSurfaceGLX::postSubBuffer(EGLint x, EGLint y, EGLint width, EGLint height)
{
    return egl::Error(EGL_SUCCESS);
}

egl::Error PbufferSurfaceGLX::querySurfacePointerANGLE(EGLint attribute, void **value)
{
    UNIMPLEMENTED();
    return egl::Error(EGL_SUCCESS);
}

egl::Error PbufferSurfaceGLX::bindTexImage(EGLint buffer)
{
    UNIMPLEMENTED();
    return egl::Error(EGL_SUCCESS);
}

egl::Error PbufferSurfaceGLX::releaseTexImage(EGLint buffer)
{
    UNIMPLEMENTED();
    return egl::Error(EGL_SUCCESS);
}

void PbufferSurfaceGLX::setSwapInterval(EGLint interval)
{
}

EGLint PbufferSurfaceGLX::getWidth() const
{
    return mWidth;
}

EGLint PbufferSurfaceGLX::getHeight() const
{
    return mHeight;
}

EGLint PbufferSurfaceGLX::isPostSubBufferSupported() const
{
    UNIMPLEMENTED();
    return EGL_FALSE;
}

}
