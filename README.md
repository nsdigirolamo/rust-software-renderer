# rust-software-renderer
A software renderer written in Rust using SDL2.
Does not have z-buffering so that's why some triangles are drawn on top of others when they should be hidden. 
Also does not have proper screen clipping so triangles will disappear too quickly if one of their vertices are off screen.
https://www.youtube.com/watch?v=oLVnrsqx9yo
