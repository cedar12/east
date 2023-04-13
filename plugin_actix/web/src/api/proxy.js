import request from './request'

/**
 * @param {string} agentId
 * @param {object} data 
 * @param {number} data.bind_port
 * @param {string} data.target_host
 * @param {number} data.target_port
 * @param {boolean} data.enable
 * @param {Array<string>} data.whitelist
 * @returns 
 */
 export const add=(agentId,data)=>{
  if(data.whitelist.length===1&&data.whitelist[0].trim()===''){
    data.whitelist=[];
  }
  return request({
    url:`/api/proxy/add/${agentId}`,
    method:'post',
    data
  })
}

/**
 * 
 * @param {number} bind_prot 
 * @returns 
 */
 export const remove=(bind_prot)=>{
  return request({
    url:`/api/proxy/remove/${bind_prot}`,
    method:'get'
  })
}

/**
 * 
 * @param {object} data 
 * @param {number} data.bind_port
 * @param {string} data.target_host
 * @param {number} data.target_port
 * @param {boolean} data.enable
 * @param {number} [data.max_rate]
 * @param {Array<string>} data.whitelist
 * @returns 
 */
export const modify=(data)=>{
  if(data.whitelist.length==1&&data.whitelist[0].trim()===''){
    data.whitelist=[];
  }
  return request({
    url:'/api/proxy/modify',
    method:'post',
    data
  })
}