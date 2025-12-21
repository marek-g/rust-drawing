# drawing

[![Crates.io Version](https://img.shields.io/crates/v/drawing.svg)](https://crates.io/crates/drawing)
[![Docs.rs Version](https://docs.rs/drawing/badge.svg)](https://docs.rs/drawing)
[![Apache-2.0 OR MIT License](https://img.shields.io/crates/l/drawing.svg)](https://github.com/marek-g/rust-drawing/blob/master/LICENSE-APACHE)

Abstraction API for 2D graphics libraries.

## Introduction

To draw something, you need to have:
- context - a drawing engine with its state
- surface - points to the destination (window or texture)
- display list - description what to draw

## Context

Context is an abstraction object over target graphics context (like OpenGL context). The context keeps all the data required for rendering. It is the main object.

The context creation is not standarized. Can be different depending on the platform or windowing library. You should first create the context outside of this drawing API and then use drawing API to wrap it.

A single context can be used for rendering into multiple windows / targets. This is the recommended way to use this library. It may work with other configurations, but this is not fully tested. Please note that in OpenGL using a single context is not the same as having multiple shared contexts.

It is reference counted, single-threaded for OpenGL and thread safe for Vulkan, mutable object (via interior mutability).

## Surface

A surface represents a render target. For OpenGL it wraps framebuffer object.

Framebuffer pointing to the window should be created outside of drawing library. Framebuffer pointing to the texture (and the texture itself) can be created with this library.

It is a short lived object. Create one for each frame and destroy it after presenting.

## Display list

A description what to draw (list of draw commands). It can be reused multiple times and with different contexts. Display list is built with display list builder.

It is reference counted, thread safe, immutable object.

## Texture

Represents an image whose data is resident in GPU memory.

It is reference counted, thread safe, immutable object.
