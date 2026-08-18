
#### Pixel

`{float}`: float representing nomber of pixels

> [!WARNING]
> don't suffix the value, don't use `px` or other metrics prefixes


#### Padding

- `{pixel} {pixel} {pixel} {pixel}` : respectivly `top right bottom left` padding values
- `{pixel} {pixel}` : respectivly `vertical` (top and bottom) and `horizontal` (left and right) padding values
- `{pixel}` : global padding value (same for all)


#### Radius
- `{pixel} {pixel} {pixel} {pixel}` : respectivly `top-left top-right bottom-left bottom-right` corners radius values
- `{pixel} {pixel}` : respectivly `top` (top-left and top-right) and `bottom` (bottom-left and bottom-right) corners radius values
- `{pixel}` : global corners radius values (same for all)


#### Length
- `fill` : the container fill all the remaining space
- `shrink` : the container fill the least amount of space
- `{pixel}` : fixed value in pixel 

#### Color
- `#RRGGBB`: Classic Hex color format **without** alpha
- `#RRGGBBAA`: Classic Hex color format **with** alpha

#### Horizontal
- `left` : Aligns text to the left
- `center` : Centers text horizontally
- `right` : Aligns text to the right

#### Vertical
- `top` : Aligns text to the top
- `center` : Centers text vertically
- `bottom` : Aligns text to the bottom