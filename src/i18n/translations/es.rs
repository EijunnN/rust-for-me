use super::TranslationSet;

pub static TRANSLATIONS: TranslationSet = TranslationSet {
    // Navigation
    nav_dashboard: "Inicio",
    nav_theory: "Teoría",
    nav_practice: "Práctica",
    nav_projects: "Proyectos",
    nav_settings: "Configuración",
    // Dashboard
    dashboard_title: "Bienvenido a Rust for Everyone",
    dashboard_welcome: "Aprende Rust de forma interactiva",
    dashboard_continue: "Continuar aprendiendo",
    dashboard_progress: "Tu progreso",
    // Theory
    theory_title: "Módulos de Teoría",
    theory_lessons: "lecciones",
    // Practice
    practice_title: "Ejercicios de Práctica",
    practice_exercises: "ejercicios",
    // Projects
    projects_title: "Proyectos Guiados",
    // Settings
    settings_title: "Configuración",
    settings_language: "Idioma",
    settings_theme: "Tema",
    settings_rust_path: "Ruta de Rust",
    // Common
    common_next: "Siguiente",
    common_previous: "Anterior",
    common_run: "Ejecutar",
    common_reset: "Reiniciar",
    common_hint: "Pista",
    common_solution: "Solución",
    common_beginner: "Principiante",
    common_intermediate: "Intermedio",
    common_advanced: "Avanzado",

    // Output panel
    output_title: "Salida",
    output_compiling: "Compilando...",
    output_compiling_running: "Compilando y ejecutando...",
    output_click_run: "Haz clic en Ejecutar para ver la salida",

    // Success panel
    success_title: "Resultado",
    success_compiled_ok: "Compilado correctamente",
    success_exercise_passed: "Ejercicio completado!",
    success_exercise_failed: "Salida incorrecta",
    success_expected: "Esperado",
    success_got: "Obtenido",
    success_keep_going: "Sigue practicando!",

    // Error explainer
    error_what: "Que significa",
    error_why: "Por que ocurre",
    error_fix: "Como solucionarlo",
    error_show_explanation: "Mostrar explicacion",
    error_hide_explanation: "Ocultar explicacion",

    // E0382 - use of moved value
    error_e0382_what: "Estas intentando usar un valor que ya fue movido a otra variable.",
    error_e0382_why: "En Rust, cuando asignas un valor a otra variable, el valor se 'mueve' y la variable original ya no es valida. Esto es parte del sistema de ownership.",
    error_e0382_fix: "Usa .clone() para crear una copia, o usa referencias (&) en lugar de mover el valor.",

    // E0502 - cannot borrow as mutable
    error_e0502_what: "Estas intentando crear una referencia mutable mientras existe una referencia inmutable.",
    error_e0502_why: "Rust no permite tener referencias mutables e inmutables al mismo tiempo para prevenir data races.",
    error_e0502_fix: "Asegurate de que las referencias inmutables ya no se usen antes de crear una referencia mutable.",

    // E0308 - mismatched types
    error_e0308_what: "Los tipos no coinciden. Rust esperaba un tipo pero encontro otro.",
    error_e0308_why: "Rust es estrictamente tipado. Cada expresion debe tener el tipo correcto.",
    error_e0308_fix: "Verifica los tipos de tus variables y expresiones. Puede que necesites una conversion explicita.",

    // E0425 - cannot find value
    error_e0425_what: "Rust no puede encontrar la variable o funcion que mencionas.",
    error_e0425_why: "La variable no fue declarada, o esta fuera de alcance, o tiene un error de escritura.",
    error_e0425_fix: "Verifica que la variable este declarada con 'let' y que el nombre este bien escrito.",

    // E0384 - cannot assign twice to immutable variable
    error_e0384_what: "Estas intentando modificar una variable que no es mutable.",
    error_e0384_why: "En Rust, las variables son inmutables por defecto. No puedes cambiar su valor una vez asignado.",
    error_e0384_fix: "Agrega 'mut' a la declaracion: let mut variable = valor;",

    // E0106 - missing lifetime specifier
    error_e0106_what: "Falta un especificador de lifetime en una referencia.",
    error_e0106_why: "Rust necesita saber cuanto tiempo vive una referencia para garantizar seguridad de memoria.",
    error_e0106_fix: "Agrega un lifetime explicito como 'a o considera usar tipos con propiedad (owned types) en su lugar.",

    // Exercise types
    exercise_type_write_code: "Escribir Codigo",
    exercise_type_fix_bug: "Corregir Error",
    exercise_type_predict_output: "Predecir Salida",
    exercise_fix_instructions: "Este codigo tiene un error. Lee el mensaje del compilador y corrige el codigo para que compile y produzca la salida esperada.",
    exercise_predict_instructions: "Observa el siguiente codigo. Sin ejecutarlo, predice cual sera la salida. Luego verifica tu respuesta.",
    exercise_compiler_error: "Error del compilador",
    exercise_predict_check: "Verificar respuesta",
    exercise_predict_correct: "Correcto! Tu prediccion fue acertada.",
    exercise_predict_incorrect: "Incorrecto. Ejecuta el codigo para ver la salida real.",
    exercise_predict_select: "Selecciona tu prediccion:",
    exercise_expected_output: "Salida esperada:",
    exercise_show_hint: "Mostrar pista",

    // Learning path / prerequisites
    path_locked: "Bloqueado",
    path_prerequisites: "Requisitos previos",
    path_complete_first: "Completa estos primero",
    path_recommended_next: "Siguiente recomendado",
    path_all_completed: "Todo el contenido completado! Excelente!",

    // Playground
    playground_title: "Rust Playground",
    playground_clear: "Limpiar",
    playground_tooltip: "Playground",
    playground_clippy: "Clippy",
    playground_mode_remote: "Remoto",
    playground_mode_local: "Local",
    playground_no_rust_hint: "Rust no esta instalado y no hay internet. Instala Rust desde rustup.rs o verifica tu conexion.",
};
