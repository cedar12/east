import request from './request'

/**
 * 
 * @param {string} username 
 * @param {string} password 
 * @returns 
 */
export const login=(username,password)=>{
  return request({
    url:'/api/login',
    method:'post',
    data:{
      username,
      password
    }
  })
}

export const loginUser=()=>{
  return request({
    url:'/api/login/user',
    method:'post'
  })
}