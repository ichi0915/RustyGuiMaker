
// The next step is to create the shaders.
pub mod vs {
	vulkano_shaders::shader! {
		ty: "vertex",
			src: "
		#version 450

		layout(location = 0) in vec3 position;
		layout(location = 1) in vec3 color;
		layout(location = 0) out vec3 frag_color;

		void main() {
			gl_Position = vec4(position, 1.0);
			frag_color = color;
		}"
	}
}

pub mod fs {
	vulkano_shaders::shader! {
		ty: "fragment",
			src: "
		#version 450

		layout(location = 0) in vec3 frag_color;
		layout(location = 0) out vec4 f_color;

		layout (push_constant) uniform PushConstants {
			int isSelected;
		} pushConstants;

		void main() {
			if (pushConstants.isSelected == 0) {
				f_color = vec4(frag_color, 1.0);
			} else {
				f_color = vec4(1.0, 1.0, 1.0, 1.0);
			}
		}
		"
	}
}

pub mod pick_vs {
	vulkano_shaders::shader! {
		ty: "vertex",
			src: "
		#version 450

		layout(location = 0) in vec3 position;
		layout(location = 1) in vec3 color;
		layout(location = 0) out vec4 frag_color;

		layout (push_constant) uniform PushConstants {
			vec4 color;
		} pushConstants;

		void main() {
			gl_Position = vec4(position, 1.0);
			frag_color = pushConstants.color;
		}"
	}
}

pub mod pick_fs {
	vulkano_shaders::shader! {
		ty: "fragment",
			src: "
		#version 450

		layout(location = 0) in vec4 frag_color;
		layout(location = 0) out vec4 f_color;

		void main() {
			f_color = frag_color;
		}
		"
	}
}
