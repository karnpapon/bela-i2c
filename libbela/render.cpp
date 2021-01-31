/**
from libbela.cpp
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
      int command_param;

      if(msg->partialMatch("/er301")){
        _arg.popInt32(command_param).isOkNoMoreArgs();
  		  oscSender.newMessage(msg->addressPattern()).add(command_param).send();	
      }
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
		printf("hhhhhhhhhhandshake received!\n");
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
