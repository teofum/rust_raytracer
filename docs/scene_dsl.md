# Scene DSL

This document defines a simple domain-specific language (DSL) used for describing scenes.

## Syntax

Each line is interpreted as a single declaration, associating an item (object, material, texture, noise generator, etc) with a label.

The line contains the label, item type, and any parameters needed to create the item:

```
<label>: <item> <parameters>
```

The parameters and order depend on the type of item being created.

#### Example

```
ball: sphere 1.5,0,0.5 5
```

### Reserved labels

Two labels are special and have a specific use in the scene: `world` and `lights`. These reserved labels may only be assigned an object.

The object assigned to `world` is the object that will be rendered. Usually, this is a list or BVH containing all the objects in the scene.

The object assigned to `lights` will be used to generate the light-sampling PDF. Usually, this is a list with the main light-emitting objects in the scene, but objects that refract or reflect a significant amount of light (such as reflecting directly off a light source) may be added to this list as well to reduce noise on caustics.

A scene _must_ assign both `world` and `lights` labels. A scene with either label left unassigned will be considered invalid and rejected by the parser.

### References

Any item already declared may be referenced within another declaration by its associated label, prefixed with the `$` symbol:

```
ball: sphere 1.5,0,0.5 5 $mat_plastic
```

### Inline declarations

Many item constructors need other items as parameters: objects may need materials, which may need textures, which may need other textures, and so on.

This means items may be declared only to be used in the constructor of another single item. In this case, inline declarations are meant to reduce unnecessary labels.

Any text inside parentheses `()` will be evaluated in-place as a separate declaration, and the item created used for the corresponding parameter:

```
mat_plastic: glossy (constant 1,0.2,0.2) (constant 0.2) 1.5
```

In the above example, two `constant` textures are created, one of `Vec4` and one of `f64` type, to be used in constructing a `mat_glossy` material.

Inline declarations work like any other declaration, and may themselves contain inline declarations. That is, inline declarations may be nested as many times as necessary:

```
lights: list (sky (emissive (constant 2,2,2)))
```

### Optional parameters

Some constructors may have one or more optional parameters, for which a default value is used if left unset. Optional parameters always go at the end of a declaration, and will be noted with brackets `[]` in constructor syntax.

## Textures

### Constant texture

**Type:** `constant`

```
constant <value>
```

The simplest texture that always returns a constant value. The output type is inferred from the type of `<value>`. Supported value types are vectors and scalars.

### Checkerboard

**Type:** `checker`

```
checker <input_1> <input_2> [<scale>]
checker_solid <input_1> <input_2> [<scale>]
```

Samples two different textures in a checkerboard pattern. A 3D solid version is also supported. The output type is inferred from the `<input_x>` textures; both inputs must have the same type. The `<scale>` parameter is optional and defaults to `1.0`.

### Interpolate

**Type:** `lerp`

```
lerp <input_1> <input_2> <t>
```

Interpolates between two different textures, based on the sampled value of a third texture `<t>`. Output type is inferred from the inputs. `<t>` must have an output type of `f64`.

### Noise texture

**Type:** `noise`

```
noise <noise_gen> [<scale>] [<sample_count>]
noise_solid <noise_gen> [<scale>] [<sample_count>]
```

Samples based on random noise. A noise generator must be provided.

### Image texture

**Type:** `image`

```
image <file_path>
```

Samples an image loaded from a file. `<file_path>` is the path to an image file, relative to the current execution directory.

### UV debug texture

**Type:** `uv_debug`

```
uv_debug
```

Returns a color based on UV coordinates. Useful for debugging mesh or object UVs.

## Materials

### Lambertian

**Type:** `lambertian`

```
lambertian <albedo>
```

A material with ideal Lambertian diffuse scattering. `<albedo>` must be a texture with a vector output type.

### Metal

**Type:** `metal`

```
metal <albedo> <roughness>
```

A material with metallic reflection. `<albedo>` and `<roughness>` are textures with a vector and scalar output type, respectively.

### Dielectric (glass)

**Type:** `glass`

```
glass [<ior>]
```

A transparent material with dielectric reflection and refraction. `<ior>` is a scalar value representing the material's index of refraction, and defaults to `1.5`.

### Glossy

**Type:** `glossy`

```
glossy <albedo> <roughness> [<ior>]
```

A glossy material that combines lambertian scattering and dielectric reflection. `<albedo>` and `<roughness>` are textures with a vector and scalar output type, respectively. `<ior>` is a scalar value representing the material's index of refraction, and defaults to `1.5`.

### Emissive

**Type:** `emissive`

```
emissive <emission_map>
```

A simple emissive material, with no reflection or scattering. `<emission_map>` must be a texture with a vector output type.

### Isotropic

**Type:** `isotropic`

```
isotropic <albedo>
```

A material that scatters light equally in all directions. Meant for constant-density volumes, and will look broken when used with surfaces. `<albedo>` must be a texture with a vector output type (solid or constant textures make more sense here, as volumes don't record UVs).

## Objects

### Sphere

**Type:** `sphere`

```
sphere <origin> <radius> <material>
```

A sphere primitive defined by an origin (center) point and a radius.

### Plane

**Type:** `plane`

```
plane <origin> <u> <v> <material>
```

A plane primitive defined by an origin (center) point and two vectors `<u>` and `<v>` from the center to the sides. The vectors must be perpendicular.

### Box

**Type:** `box`

```
box <origin> <size> <material>
```

An axis-aligned box primitive defined by an origin (center) point and a size with x, y and z components.

### Mesh

**Type:** `mesh`

```
mesh <file_path> <material>
```

A triangle mesh loaded from a .obj file. `<file_path>` is the path to a .obj file, relative to the current execution directory.

### Transform

**Type:** `transform`

```
transform <object> <...transform_prop>
```

A container that allows applying a transform matrix to an object. `<...transform_prop>` is a list of key-value pairs with any non-null transformations applied:

- `t=<vec>`: Translation
- `s=<vec|f64>`: Scale, may be a vector (non-uniform) or scalar (uniform)
- `rx=<f64>`: Rotation along the X axis
- `ry=<f64>`: Rotation along the Y axis
- `rz=<f64>`: Rotation along the Z axis

### Object list

**Type:** `list`

```
list <...object>
```

A container for a set of objects. Takes a list of objects as parameter.

### Bounding Volume Hierarchy

**Type:** `bvh`

```
bvh <...object>
```

A container for a set of objects with a bounding-volume hierarchy, used to improve performance with large lists of disjoint objects. Takes a list of objects as parameter.

### Sky

**Type:** `sky`

```
sky <emission_map>
```

A utility object for diffuse sky lighting. Rendered as a sphere of infinite radius. As an emissive-only object, it doesn't allow specifying a material and instead takes a texture with a vector output type as the emission map for an implicit emissive material.

### Sun

**Type:** `sun`

```
sun <direction> <emission_map>
```

A utility object for ideal directional lighting. Rendered as a point of light at infinity. As an emissive-only object, it doesn't allow specifying a material and instead takes a texture with a vector output type (usually a constant, as it doesn't report UVs) as the emission map for an implicit emissive material.

### Volume

**Type:** `volume`

```
volume <boundary> <material> <density>
```

A constant-density volume defined by some boundary object. Only objects with a convex shape are supported.

## Noise generators

### Perlin noise

**Type:** `perlin`

```
perlin
```

A 3D perlin noise generator.
