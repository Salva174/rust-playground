//* [x] In Error Handling einlesen ( https://burntsushi.net/rust-error-handling/ )
//
// * [ ] Custom Error Typ im Backend für load_configuration_from_environment_variables() einführen.
//   Das erlaubt uns Kontext hinzuzufügen, wobei ein Fehler geflogen ist. In der Funktion hatten wir noch TODOs stehen, die jetzt damit resolved werden können.
//   Insbesondere sollte dein Error Typ z.B. die Information beinhalten, welche Umgebungsvariable wir gerade verarbeitet haben, als der Fehler geflogen ist.
//   Du solltest die Fehlerausgabe sehen können, indem du z.B. `PIZZERIA_BACKEND_BIND_HOST=quatsch123 cargo run --bin pizzeria-backend` ausführst.
//
// * [ ] Falls du damit durchkommst, kannst du versuchen, das auch im Frontend in der `http.rs` einzuführen. Da wäre das Ziel, die Fehler besser debuggen zu können, die da fliegen.
//   Als wir das mit `PIZZERIA_FRONTEND_BACKEND_HOST=pi-zzeria.detss.corpintra.net cargo run --bin raw` gemacht haben, dachte ich eigentlich dass das schief gehen müsste, weil dein Frontend-Code jetzt versucht, das als `SocketAddr` zu parsen.
//   Den Fehler haben wir nicht bekommen, also könnte es sein, dass dein Code die Umgebungsvariablen irgendwie doch noch nicht ausliest.
//
//   Sonst kannst du die Verbindung auch mal mit `PIZZERIA_FRONTEND_BACKEND_HOST=53.247.103.233 cargo run --bin raw` (wenn das Backend auf dem Pi wieder mit der Bind Address 0.0.0.0 gestartet ist). Das ist die Public IP-Adresse unter der der Pi erreichbar sein sollte.
//   Das Frontend *sollte* funktionieren, wenn bei `curl 53.247.103.233:3333` die Liste der Toppings herauskommt (und nicht `curl: (52) Empty reply from server`).
//

use std::env::VarError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    NotUnicode {
        key: String,
        source: VarError,
    },
    Parse {
        key: String,
        value: String,
        source: Box<dyn Error + Send + Sync + 'static>
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotUnicode { key, .. } =>
                write!(f, "ENV {key} ist nicht gültiges Unicode"),
            ConfigError::Parse { key, value, source } =>
                write!(f, "ENV {key}='{value}' konnte nicht geparst werden: {source}")
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::NotUnicode { source, .. } => Some(source),
            ConfigError::Parse { source, ..} => Some(source.as_ref()),
        }
    }
}
