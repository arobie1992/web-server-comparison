import Vapor

struct HttpReq: Content {
    var url: String
}

struct Info: Content {
	var id: uint
    var status: String
	var data: Data
}

struct Data: Content {
	var requestors: Array<String>
}

func routes(_ app: Application) throws {
    app.post("call") { req -> Info in
        let httpReq = try req.content.decode(HttpReq.self)

        let url = URL(string: httpReq.url)!        

        var resp: ByteBuffer?
        let group = DispatchGroup()
        group.enter()
        let task = URLSession.shared.dataTask(with: url) {(data, response, error) in
            guard let data = data else { return }
            resp = ByteBuffer(data: data)
            group.leave()
        }
        task.resume()
        // Need to wait for the callback to finish so we have resp
        group.wait()
        
        let info = try JSONDecoder().decode(
            Info.self,
            from: resp!
        )

        return info
    }
}
