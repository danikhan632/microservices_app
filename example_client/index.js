// import grpc from "grpc";
// import protoLoader from "@grpc/proto-loader";

const grpc = require("grpc");
const protoLoader = require("@grpc/proto-loader");
const PROTO_PATH = "./auth.proto";
const SERVER_URI = "localhost:50051";

const definition = protoLoader.loadSync(
    PROTO_PATH
  );

  const serviceProto = grpc.loadPackageDefinition(definition).auth;
//   console.log(serviceProto);
  const client = new serviceProto.Auth(SERVER_URI, grpc.credentials.createInsecure());
    const userName = "amdwain";
    const password = "admn";

    client.ValidateAuth({username: userName, password: password}, (error, response) => {
        if (error) {
          console.error(error);
        } else {
          console.log(response);
        }
      });

      // client.CreateNewUser({username: userName, password: password}, (error, response) => {
      //   if (error) {
      //     console.error(error);
      //   } else {
      //     console.log(response);
      //   }
      // });
  