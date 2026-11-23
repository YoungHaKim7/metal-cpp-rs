# Fixes applied to examples/gui/mod.rs

Symptom: every GUI example opened a window but showed only the (near-black)
clear color — no rects, circles, lines, or text were drawn.

## Bug 1 — missing vertex-stage uniform binding (black screen)

The default shader's vertex function reads its uniforms from `[[buffer(1)]]`:

```metal
vertex GuiOut vertex_main(const device GuiVertex* vertices [[buffer(0)]],
                          constant float4* uniforms [[buffer(1)]], ...)
{
    float2 size = uniforms[0].xy;          // screen size, pixel -> NDC
    ...
}
```

but `App::render()` only bound that buffer at the **fragment** stage
(`setFragmentBuffer(uniform_buffer, 0, 0)`) and never at the **vertex**
stage. With `[[buffer(1)]]` unbound, `uniforms[0].xy` read garbage, so the
pixel -> NDC division produced garbage/NaN positions and every triangle was
clipped away — a black window with only the clear color visible.

Fix (two places in `render()`, the `frame.verts` draw and the
`frame.text_verts` draw):

```rust
encoder.setVertexBuffer(self.vertex_buffer.as_ref(), 0, 0);
encoder.setVertexBuffer(self.uniform_buffer.as_ref(), 0, 1);   // <-- added
encoder.setFragmentBuffer(self.uniform_buffer.as_ref(), 0, 0);
```

(The `draw_custom` path already had this binding and was unaffected, which
is why the symptom was "just a black window" for the default shapes/text.)

## Bug 2 — text pushed one vertex per font pixel

`Frame::text()` pushed a single `Vertex` for every lit bit of the 5x7
bitmap font. Since the text pass draws with
`MTLPrimitiveType::PrimitiveTypeTriangle`, consecutive triples of those
single pixels were rasterised as arbitrary garbage triangles instead of
glyph shapes.

Fix: each lit font pixel now emits a filled quad (2 triangles, 6 vertices)
covering its `size x size` cell — same vertex pattern `Frame::rect` uses.

## Minor — text vertex capacity

`text_verts` initial capacity raised from 2048 to 8192 because text now
emits 6 vertices per lit font pixel (avoids per-frame reallocation).

## Files changed

- `examples/gui/mod.rs` — all three fixes above (search for `FIX:`)

No other files were modified; the crate itself (`src/`) was correct — the
bugs were in the example GUI runtime only.
