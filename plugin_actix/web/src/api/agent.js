import request from './request'

export const agents=()=>{
  return request({
    url:'/api/agent',
    method:'get'
  })
}

/**
 * 
 * @param {object} data 
 * @param {string} data.id
 * @param {string} data.name
 * @returns 
 */
export const add=(data)=>{
  return request({
    url:'/api/agent/add',
    method:'post',
    data
  })
}

/**
 * 
 * @param {string} id 
 * @returns 
 */
export const remove=(id)=>{
  return request({
    url:`/api/agent/remove/${id}`,
    method:'get'
  })
}