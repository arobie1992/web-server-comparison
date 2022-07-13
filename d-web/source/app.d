import vibe.vibe;
import std.net.curl;
import std.conv;
import asdf;

// Interface is required to pass impl to listenHTTP
interface IRest
{
    struct Info 
    {
	    int id;
        string status;
	    Data data;
    }

    struct Data 
    {
	    string[] requestors;
    }

    @path("/call")
    Info postReq(string url);
}

class Rest: IRest
{
    /*
    Post data as:
        { "url": "http://localhost:8080/resp" }
    */
    Info postReq(string url)
    {
        auto resp = to!string(get(url));
        auto info = resp.deserialize!Info;
        return info;
    }
}

void main()
{
    auto router = new URLRouter;
    router.registerRestInterface(new Rest);

    auto settings = new HTTPServerSettings;
    settings.bindAddresses = ["::1", "127.0.0.1"];
    settings.port = 8082;
    auto listener = listenHTTP(settings, router);
    scope (exit)
	{
		listener.stopListening();
	}
    runApplication();
    
}
