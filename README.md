# XORG Crypt
## Compilacón
```
cd xorg_crypt && cargo build --release
```
## Uso
### Encriptar
```bash
./target/release/xorg_crypt encrypt "Mensaje" "Clave"
```
### Desencriptar
```bash
./target/release/xorg_crypt decrypt "Binario" "Clave"
```