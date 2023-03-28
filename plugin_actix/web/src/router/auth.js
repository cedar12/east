import router from './index'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

import {loginUser} from '@/api/user'

const whitelist=['login']

router.beforeEach(async (to, from) => {
  NProgress.start()
  if (whitelist.includes(to.name)) {
    NProgress.done()
    return true;
  }
  try{
    const info=await loginUser();
    return true;
  }catch(e){
  
  }
  return {name:'login'}
})

router.afterEach(() => {
  NProgress.done()
})