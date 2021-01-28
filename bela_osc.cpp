/**
\example OSC/render.cpp

Open Sound Control
------------------

This example shows an implementation of OSC (Open Sound Control) which was 
developed at UC Berkeley Center for New Music and Audio Technology (CNMAT).

It is designed to be run alongside resources/osc/osc.js.
For the example to work, run in a terminal on the board
```
node /root/Bela/resources/osc/osc.js
```

In `setup()` an OSC message to address `/osc-setup`, it then waits 
1 second for a reply on `/osc-setup-reply`.

After that, OSC communication takes place in the on_receive() callback,
which is called every time a new message comes in.
*/

#include <Bela.h>
#include <libraries/OscSender/OscSender.h>
#include <libraries/OscReceiver/OscReceiver.h>

OscReceiver oscReceiver;
OscSender oscSender;
int localPort = 7562;
int remotePort = 7563;
const char* remoteIp = "127.0.0.1";

// parse messages received by the OSC receiver
// msg is Message class of oscpkt: http://gruntthepeon.free.fr/oscpkt/
bool handshakeReceived;
void on_receive(oscpkt::Message* msg, void* arg)
{
	if(msg->match("/osc-setup-reply"))
		handshakeReceived = true;
	else if(msg->match("/osc-test")){
		oscSender.newMessage("/osc-acknowledge").add(std::string("OSC message received")).send();
	} else {
		if(handshakeReceived){
			auto _arg = msg->arg();
			std::string str;
			int iArg1;
      int iArg2;
      std::string sArg1;

			_arg.popInt32(iArg1);
			_arg.popStr(sArg1);
      _arg.popInt32(iArg2);
		oscSender.newMessage(msg->addressPattern()).add(iArg1).add(sArg1).add(iArg2).send();	
		}
	} 
}

bool setup(BelaContext *context, void *userData)
{
	oscReceiver.setup(localPort, on_receive);
	oscSender.setup(remotePort, remoteIp);

	// the following code sends an OSC message to address /osc-setup
	// then waits 1 second for a reply on /osc-setup-reply
	oscSender.newMessage("/osc-setup").send();
	int count = 0;
	int timeoutCount = 10;
	printf("Waiting for handshake ....\n");
	while(!handshakeReceived && ++count != timeoutCount)
	{
		usleep(100000);
	}
	if (handshakeReceived) {
		printf("handshake received!\n");
	} else {
		printf("timeout!: you didn't start osc server.\n");
		return false;
	}
	return true;
}

void render(BelaContext *context, void *userData)
{

}

void cleanup(BelaContext *context, void *userData)
{

}
