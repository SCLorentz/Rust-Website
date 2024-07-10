# RockScript Website

## some ideas for domains:

- rockscript.code
- rockscript.dev

## subdomains

- help.rockscript.dev
- awesome.rockscript.dev

## Commands

use `./build.sh` to build and start the server

if this dosen't work, use `chmod +x build.sh`

`/home/sclorentz/Downloads/upx-4.2.4-amd64_linux/upx --best --lzma target/debug/rok-page` --> compact the compiled file

## Input do terminal

- `echo * "message"` --> printa uma mensagem no terminal do navegador de todos os usuarios, o valor '*' pode ser substituido pelo ip do usuário
- `ban [ip] "reason" 1` --> bane um usuário pelo ip (retorna o erro 403 e indica a razão), o valor representado por "1" indica uma hora
- `redirect [ip] https://example.com` --> redireciona um usuário para uma pagina especifica
- `quick [ip] "reason"` --> desconecta o usuário da página
- `stop` --> para o servidor
- `location [ip]` --> retorna a localização do dispositivo baseado ip
- `stats` --> obtem os status do servidor
