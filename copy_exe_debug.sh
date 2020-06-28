
ssh pi 'rm ~/rasp_pi'
ssh pi2 'rm ~/rasp_pi'

scp ./target/arm-unknown-linux-gnueabi/debug/rasp_pi pi:~
scp ./target/arm-unknown-linux-gnueabi/debug/rasp_pi pi2:~

