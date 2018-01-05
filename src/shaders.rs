/* Define the vertex and fragment shaders here. */

/* Source for triangle vertex shader. */
pub const TRI_VERT: &str = r#"
    #version 150
    
    in vec3 position;
    in vec3 normal;

    out vec3 v_position;
    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        v_position = (modelview * vec4(position, 1.0)).xyz;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;

/* Source for triangle fragment shader. */
pub const TRI_FRAG: &str = r#"
    #version 140
    
    in vec3 v_position;
    in vec3 v_normal;
    out vec4 color;

    uniform vec3 u_light;
    uniform vec3 surface_color;

    const vec3 ambient_color = vec3(0.1, 0.1, 0.1);
    const vec3 diffuse_color = vec3(0.8, 0.8, 0.8);
    const vec3 specular_color = vec3(1.0, 1.0, 1.0);
    
    void main() {
        float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

        vec3 camera_dir = normalize(-v_position);
        vec3 half_direction = normalize(normalize(u_light) + camera_dir);
        float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 32.0);

        color = vec4((ambient_color + diffuse * diffuse_color + specular * specular_color) * surface_color, 1.0);
    }
"#;

pub const LINE_VERT: &str = r#"
    #version 150
    
    in vec3 position;
    out vec3 v_position;

    uniform mat4 perspective;
    uniform mat4 view;

    void main() {
        mat4 modelview = view;
        v_position = (modelview * vec4(position, 1.0)).xyz;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;

pub const LINE_FRAG: &str = r#"
    #version 140
    
    in vec3 v_position;
    out vec4 color;
    
    uniform vec3 surface_color;

    void main() {
        color = vec4(surface_color, 0.2);
    }
"#;