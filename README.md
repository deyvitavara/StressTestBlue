# StressTestBlue

StressTestBlue es una herramienta de calidad para ejecutar **pruebas de rendimiento** en APIs REST de forma rápida, ligera y sin configuraciones eternas.  
Está desarrollada en **Rust** para garantizar **velocidad, seguridad y bajo consumo de recursos**.

> Proyecto de **código abierto (MIT)**, desarrollado por **Deyvi Távara** como parte de la plataforma [OpenBlue](https://openblue.blue).

---

## 🚀 Características
- Implementado 100% en **Rust** 🦀.
- Soporta **GET**, **POST** y otros métodos HTTP.
- Cuatro modos listos para usar:
  - **load** (carga sostenida)
  - **spike** (picos repentinos)
  - **stress** (llegar al límite)
  - **soak** (resistencia a largo plazo)
- Parámetros desde CLI: `--endpoint`, `--method`, `--concurrency`, `--duration`, `--mode`.
- Funciona en **Windows**, **Linux** y **macOS**.
- Pensado para **QA**, **DevOps** y **equipos de producto** que necesitan resultados reales sin overhead.

---

## 📦 Instalación

### Requisitos
- [Rust](https://www.rust-lang.org/tools/install) (toolchain estable).

### Clonar y compilar
```bash
git clone https://github.com/TU_USUARIO/StressTestBlue.git
cd StressTestBlue
cargo build --release
```

El binario quedará en:
```
target/release/stresstestblue
```

> Sugerencia: añade `target/release` a tu `PATH` o copia el binario a una carpeta incluida en tu `PATH`.

---

## 🖥 Uso Rápido (CLI)

### 1) Carga sostenida (Load)
```bash
./stresstestblue --endpoint https://jsonplaceholder.typicode.com/posts --method GET --concurrency 10 --duration 20 --mode load
```

### 2) Picos (Spike)
```bash
./stresstestblue --endpoint https://jsonplaceholder.typicode.com/posts --method GET --concurrency 2 --duration 20 --mode spike
```

### 3) Estrés (Stress)
```bash
./stresstestblue --endpoint https://jsonplaceholder.typicode.com/posts --method GET --concurrency 5 --duration 30 --mode stress
```

### 4) Resistencia (Soak)
```bash
./stresstestblue --endpoint https://jsonplaceholder.typicode.com/posts --method GET --concurrency 5 --duration 300 --mode soak
```

> Cambia `--method` a `POST` y añade `--body` (si tu CLI lo soporta) para pruebas con payload.  
> Para entornos que requieren seguridad, puedes extender con **Bearer token / API keys**.

---

## 📂 Arquitectura del Proyecto

El código está organizado por **capas** para mantener claridad y extensibilidad:

```
src/
│   main.rs              → Punto de entrada del programa.
│   cli.rs               → Parseo de argumentos y validaciones de CLI.
│   client.rs            → Cliente HTTP para realizar solicitudes.
│   config.rs            → Estructuras y carga de configuración.
│   logger.rs            → Logging a consola/archivo.
│   metrics_writer.rs    → Escritura y persistencia de métricas (CSV/JSON, extensible).
│
├── domain/              → Modelos y tipos del dominio.
│   ├── mod.rs
│   └── test_config.rs   → Config específico para escenarios de prueba.
│
└── engine/              → Motor de ejecución de pruebas.
    ├── concurrency.rs   → Estrategias de concurrencia (hilos/async).
    ├── dispatcher.rs    → Orquestación de peticiones y control del flujo.
    ├── load.rs          → Implementación del modo Carga.
    ├── metrics.rs       → Cálculo/agrupación de métricas (latencia, throughput, errores).
    ├── modos.rs         → Definición/contratos de los modos de prueba.
    ├── rest.rs          → Lógica para llamadas REST (HTTP).
    ├── results.rs       → Resultado agregado y formateo.
    ├── soak.rs          → Implementación del modo Resistencia.
    ├── spike.rs         → Implementación del modo Pico.
    ├── stress.rs        → Implementación del modo Estrés.
    └── utils.rs         → Utilidades comunes.
```

### Decisiones de diseño
- **Separación nítida** entre CLI, dominio y motor → facilita mantenimiento y testing.
- **Modos desacoplados** → agregar un nuevo modo es tan simple como crear un módulo y registrarlo.
- **Métricas centralizadas** → todas las estrategias reportan en un formato unificado.
- **Cliente HTTP encapsulado** → cambiar la librería HTTP es trivial.

---

## 📊 Métricas (qué reporta)
- **Latencia** (p50, p90, p95, p99) — si habilitas percentiles.
- **Throughput** (req/seg).
- **Errores** por tipo/código HTTP.
- **Tiempo total** y **peticiones completadas**.
- Salida **CSV/JSON** (según implemente `metrics_writer.rs`).

> Tip: integra la salida con **Grafana/Looker/PowerBI** para dashboards.

---

## 🛠 Extender / Personalizar
- **Auth**: añade `Authorization: Bearer <token>` o API keys en `client.rs`.
- **Body/Headers**: expón flags en `cli.rs` y propágalos a `client.rs`.
- **Reportes**: implementa nuevos writers
- **Modos nuevos**: crea `engine/<tu_modo>.rs` y añádelo al `dispatcher`.

---

## 🧪 Ejecución
- Ejecuta pruebas de **smoke** con baja concurrencia en PRs.
- Corre **stress** y **soak** en jobs nocturnos o ambientes dedicados.
- Exporta métricas como artefactos del pipeline.

---

## 🧩 Roadmap (ideas)
- Soporte nativo para **headers y payloads** desde archivo.
- **Percentiles configurables** y histogramas.
- Exportación directa a **Prometheus/InfluxDB**.
- Reintentos con **exponencial backoff**.

---

## 👥 Contribuir
¡Contribuciones bienvenidas!  
Haz un **fork**, crea una **branch** y envía un **pull request**.  
También puedes abrir **issues** con mejoras o bugs.

---

## 📜 Licencia
Este proyecto está bajo la **licencia MIT**.  
Puedes usarlo, modificarlo y distribuirlo libremente citando la autoría original.

---

## 📣 Créditos
Desarrollado por **Deyvi Távara**  
https://openblue.blue
