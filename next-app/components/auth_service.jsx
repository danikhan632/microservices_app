// import grpc from "grpc";
// import protoLoader from "@grpc/proto-loader";
import { useEffect, useState } from "react";
import { grpc } from "@improbable-eng/grpc-web";
import { Auth, AuthRequest } from "./auth_pb";
// const SERVER_URI = "localhost:50051";
// const call = new AuthRequest("admin", "admin");

//  grpc.unary(Auth.ValidateAuth, {
//   request: call,
//   host: "http://localhost:50051",
//   //    debug: true,
//   onEnd: (res) => {
//     console.log(res);
//     return res;
//   },
// });



export const useFetch = () => {
  const call = new AuthRequest("admin", "admin");
  // const [grpcRes, setGrpcRes] = useState();
  const grpcRes="hi"
  useEffect(() => {
    grpc.unary(Auth.ValidateAuth, {
      request: call,
      host: "http://localhost:50051",
      //    debug: true,
      onEnd: (res) => {
        console.log(res);
        const { status, statusMessage, message } = res;
        // if (message) setGrpcRes(message.toObject());
      },
    });
    return () => {};
  }, []);
  return grpcRes;
};