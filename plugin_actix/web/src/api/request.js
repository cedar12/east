import axios from 'axios'
import { MessagePlugin } from 'tdesign-vue-next';

const instance = axios.create({
  baseURL: '/',
  timeout: 5000
});
 

instance.interceptors.request.use(function (config) {
  let token = localStorage.getItem("east-token");
  if (token) {
    config.headers = {
      "Authorization": token
    }
  }
  return config;
}, function (error) {
  return Promise.reject(error);
});
 
instance.interceptors.response.use(function (response) {
  console.log(response);
  if(response.data.code!=2000){
    MessagePlugin.error(response.data.info);
    return Promise.reject(new Error(response.data.info))
  }
  return response.data;
}, function (error) {
  return Promise.reject(error);
});
 
 
export default instance;