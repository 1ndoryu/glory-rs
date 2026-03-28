# Plan: Glory-rs Framework como Submódulo — 253A-18

## Objetivo
Crear un mini framework interno (glory-rs) como repositorio git independiente y añadirlo como submódulo al proyecto glory-rust-template. Contiene componentes UI atómicos y estilos reutilizables.

## Componentes agnósticos identificados
- **UI Atoms**: Boton, Input, Select, Textarea, Modal + barrel export (index.ts)
- **CSS Design System**: Componentes.css (estilos de botones, campos, modal, animaciones)
- **Dependencias**: React (peer), lucide-react (peer, solo Modal)

## Estructura del submódulo
```
glory-rs-framework/
  README.md
  package.json          (peerDependencies: react, lucide-react)
  frontend/
    componentes/
      ui/
        Boton.tsx
        Input.tsx
        Select.tsx
        Textarea.tsx
        Modal.tsx
        index.ts
    estilos/
      Componentes.css
```

## Pasos de ejecución
1. Crear directorio `glory-rs-framework` como sibling del proyecto
2. Init git, crear estructura, copiar archivos con paths adaptados
3. Commit inicial
4. En glory-rust-template: `git submodule add ../glory-rs-framework glory-rs`
5. Configurar Vite: alias `@glory` -> `../glory-rs/frontend`, `server.fs.allow: ['..']`
6. Configurar tsconfig: `paths` con `@glory/*`
7. Actualizar imports en 8 archivos: `'./ui'` -> `'@glory/componentes/ui'`
8. Eliminar originales: `frontend/src/componentes/ui/` y `frontend/src/estilos/Componentes.css`
9. Validar: type-check + get_errors
10. Commit y push

## Nota sobre GitHub
Sin gh CLI disponible. El submódulo se referencia con path local relativo. El usuario necesitará:
1. Crear repo en GitHub (ej: `1ndoryu/glory-rs-framework`)
2. Push del repo local
3. Actualizar URL en `.gitmodules`
