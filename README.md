# 🌌 Proyecto 3: Space Travel

---

## 🎮 Controles

### Movimiento de cámara

- `A` / `D` : mover cámara en X (izquierda / derecha).
- `W` / `S` : mover cámara en Y (arriba / abajo).
- ⬆️ / ⬇️ : **zoom in / zoom out**.



### Instant Warp (con transición animada)

Teclas numéricas:

- `1` → Ir al **Sol**.
- `2` → Ir al **Planeta rocoso**.
- `3` → Ir al **Planeta gaseoso con anillo**.
- `4` → Ir al **Planeta cibernético**.
- `5` → Ir al **Planeta de lava**.


### Extra

- `P` → Guardar captura de pantalla del framebuffer (`space_render.png`).

---

## 🏗 Estructura del proyecto

Principales módulos (pueden variar según la versión final):

- `main.rs`  
  Configura la ventana, la cámara, el sistema solar y el bucle principal.
- `framebuffer.rs`  
  Implementa el framebuffer, color buffer, z-buffer y guardado de imagen.
- `vertex.rs`, `fragment.rs`, `triangle.rs`  
  Etapas del pipeline de render:
  - transformación de vértices,
  - ensamblado de triángulos,
  - rasterizado de fragmentos.
- `shaders.rs`  
  Lógica de shading para diferentes tipos de planetas / materiales / anillos.
- `obj.rs`  
  Carga de modelos `.obj` (esfera, anillo, etc.).
- `matrix.rs`  
  Utilidades para matrices de transformación (modelo).

---

## ▶️ Cómo ejecutar

Requisitos:

- [Rust](https://www.rust-lang.org/) instalado.
- Dependencias de `raylib` según tu sistema (librerías nativas).

Pasos:

```bash
# Clonar este repositorio
git clone https://github.com/can23384/Proyecto-3-Space-Travel
cd Proyecto-3-Space-Travel

# Compilar y ejecutar
cargo run --release
```


## 🎬 Video de demostración



[![Ver video en YouTube](https://i9.ytimg.com/vi/FqBhufwYCSY/mqdefault.jpg?sqp=CKDDxMgG-oaymwEmCMACELQB8quKqQMa8AEB-AHoBYAC4AOKAgwIABABGEMgXChlMA8=&rs=AOn4CLDk79TDydDxnefhATqKJxdl_ukWQg)](https://youtu.be/FqBhufwYCSY)



