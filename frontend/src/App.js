import logo from './logo.svg';
import './App.css';
const grpc = require("grpc");
const protoLoader = require("@grpc/proto-loader");
function App() {



const PROTO_PATH = "./auth.proto";
const SERVER_URI = "localhost:50051";



const valida =() => {
  const definition = protoLoader.loadSync(
    PROTO_PATH
  );

  const serviceProto = grpc.loadPackageDefinition(definition).auth;
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
};

     function componentDidMount() {
        // Runs after the first render() lifecycle
        console.log("componentDidMount");
        valida();
    
      }
    

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
