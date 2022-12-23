import {useFetch} from '../../components/auth_service';
export default function handler(req, res) {
  const grpcRes = useFetch();
  res.status(200).json({ name: grpcRes })
}
