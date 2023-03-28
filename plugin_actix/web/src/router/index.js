import {createRouter,createWebHashHistory} from 'vue-router'

const routes=[
  
  {
    path:'/login',
    name:'login',
    component:()=>import('@/views/login/index.vue')
  },
  {
    path:'/',
    name:'layout',
    // redirect:'/dashboard',
    component:()=>import('@/views/layout/index.vue'),
    children:[
      {
        path:'/dashboard',
        name:'dashboard',
        meta:{
          title:'仪表盘',
          icon:'dashboard'
        },
        component:()=>import('@/views/dashboard/index.vue')
      },
      {
        path:'/agent',
        name:'agent',
        meta:{
          title:'资源列表',
          icon:'server'
        },
        component:()=>import('@/views/agent/index.vue')
      }
    ]
  },
  {
    path:'/',
    redirect:'/dashborad'
  }
]

export default createRouter({
  routes,
  history:createWebHashHistory()
})