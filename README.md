## Rustonic

A simple web app for playing music from a server via the Subsonic API.

Front-end only for now. Work in progress.

### Try It Out

Make sure you have `trunk` installed:  
`cargo install --locked trunk`

Clone this repository:  
`git clone https://github.com/IsaacHorvath/rustonic.git`

Launch the server and open in your browser:  
`trunk serve --open`

You won't be able to play any audio at this point - just browse a demo library and add songs to a dummy queue.

### Design Notes

- Built entirely in Rust, HTML, and CSS
- Uses [Yew](https://yew.rs/), [Trunk](https://trunkrs.dev/), and a bit of [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) so far
- Central event bus (Yew Agent) caches albums, artists, and songs
- Independent Components display this data on an artist tab, and album/song lists

### Todo:

[ ] Implement play queue  
[ ] Rework CSS with flexboxes  
[ ] Implement subsonic REST API  
[ ] Fix silly default colours :P  
[ ] Implement the actual player  
[ ] Proper caching  
[ ] Build a backend?
