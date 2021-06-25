in vec2 v_uv;
out vec4 frag;

uniform sampler2D tex;

// 0-255 sRGB  from  0-1 linear
vec3 srgb_from_linear(vec3 rgb) {
  bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
  vec3 lower = rgb * vec3(3294.6);
  vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
  return mix(higher, lower, vec3(cutoff));
}

// 0-255 sRGBA  from  0-1 linear
vec4 srgba_from_linear(vec4 rgba) {
  return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
}

void main() {
  // this rotates the texture so the letters face up, still haven't figured out why they are rotated
  vec2 rot_uv = vec2(v_uv.x, 1.0 - v_uv.y);
  frag = texture(tex, rot_uv);

  // Unmultiply alpha:
  if(frag.a > 0.0) {
    frag.rgb /= frag.a;
  }

  // Empiric tweak to make e.g. shadows look more like they should:
  frag.a *= sqrt(frag.a);

  // To gamma:
  frag = srgba_from_linear(frag) / 255.0;

  // Premultiply alpha, this time in gamma space:
  if(frag.a > 0.0) {
    frag.rgb *= frag.a;
  }

}
