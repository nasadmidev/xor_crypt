# XOR Crypt
## Compilación
```
cd xor_crypt && cargo build --release
```
## Uso
### Encriptar
```bash
./target/release/xor_crypt encrypt "Mensaje" "Clave"
```
### Desencriptar
```bash
./target/release/xor_crypt decrypt "Binario" "Clave"
```
## Ejemplo
```bash
$ ./target/release/xor_crypt encrypt "Mensaje oculto" "clave secreta"
# 0010111000001001000011110000010100000100010010100001011001000101000011000001000100010000000110000001010100001100
$ ./target/release/xor_crypt decrypt "0010111000001001000011110000010100000100010010100001011001000101000011000001000100010000000110000001010100001100" "clave secreta"
# Mensaje oculto
```
