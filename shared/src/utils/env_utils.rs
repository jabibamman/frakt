use std::env;

/// Tente de récupérer une variable d'environnement et de la convertir en `u16`.
/// Retourne `Ok` avec la valeur convertie ou `Err` avec un message d'erreur.
pub fn get_env_var_as_u16(var_name: &str) -> Result<u16, String> {
    match env::var(var_name) {
        Ok(value) => value.parse::<u16>().map_err(|e| {
            format!(
                "Erreur lors de la conversion de la variable d'environnement {} en u16: {}",
                var_name, e
            )
        }),
        Err(_) => Err(format!("Variable d'environnement {} non définie", var_name)),
    }
}
