open Lwt
(* I think Dream comes packaged with Httpaf (https://github.com/inhabitedtype/httpaf)
   which is supposed to be able to make HTTP requests but I didn't have much figuring out how to *)
open Cohttp_lwt_unix

type http_request = { 
  url : string 
} [@@deriving yojson]

(* data needs to be declared before info so it can be used 
   look into how to get around this *)
type data = { 
  requestors : string list 
} [@@deriving yojson]

type info = {
  id : int;
  status : string;
  data : data;
} [@@deriving yojson]

let call_url url =
  Client.get (Uri.of_string url) 
  >>= fun (_resp, body) ->
    body 
    |> Cohttp_lwt.Body.to_string 
    >|= Yojson.Safe.from_string 
    >|= info_of_yojson

let handle_call request =
  (* let%lwt x = fn() in y... tells fn()'s returned promise to wait to complete so 
     x has the return value of the promise rather than the promise letting y... 
     operate on the value *)
  let%lwt body = Dream.body request in
  let http_request =
    Yojson.Safe.from_string body
    |> http_request_of_yojson
  in
  let%lwt resp = (call_url http_request.url) in
  yojson_of_info resp
  |> Yojson.Safe.to_string 
  |> Dream.json 

let () =
  Dream.run ~port: 8084
  @@ Dream.logger
  (* CORS check from example: disabled for now *)
  (* @@ Dream.origin_referrer_check *)
  @@ Dream.router [
    Dream.post "/call" handle_call;
  ]