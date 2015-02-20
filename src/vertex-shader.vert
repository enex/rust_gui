/*uniform vec2 viewSize
in vec2 vertex;
in vec2 tcoord;
out vec2 ftcoord;
out vec2 fpos;

void main(void) {
	ftcoord = tcoord;
	fpos = vertex;
	gl_Position = vec4(2.0*vertex.x/viewSize.x - 1.0, 1.0 - 2.0*vertex.y/viewSize.y, 0, 1);
}
*/
#version 120
attribute vec2 a_Pos;
attribute vec4 a_Color;
varying vec4 v_Color;
    
void main() {
	v_Color = a_Color;
	gl_Position = vec4(a_Pos, 0.0, 1.0);
}
