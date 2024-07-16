flash:
	openocd -f openocd.cfg -c "program target/thumbv7em-none-eabihf/debug/led-demo preverify verify reset exit"