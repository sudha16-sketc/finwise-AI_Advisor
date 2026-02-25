import axios from 'axios';

const BACKEND_API = '/api'; // Proxied through Vite

export const callSorobanContract = async (functionName, args = []) => {
  try {
    const response = await axios.post(`${BACKEND_API}/call-contract`, {
      functionName,
      args,
    });
    return response.data.result;
  } catch (error) {
    throw new Error(error.response?.data?.error || 'Soroban contract call failed');
  }
};

export const getContractAddress = async () => {
  try {
    const response = await axios.get(`${BACKEND_API}/contract-address`);
    return response.data.contractId;
  } catch (error) {
    return null;
  }
};
