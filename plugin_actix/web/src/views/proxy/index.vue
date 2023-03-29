<template>
  <div class="proxy-container">
    <t-button @click="state.visible=true">新增</t-button>
  <t-base-table
  bordered
  footerAffixedBottom
  headerAffixedTop
  hover
  stripe
        row-key="index"
        :data="state.data"
        :columns="columns"
      ></t-base-table>
      <ProxyForm v-model="state.visible"/>
    </div>
</template>
<script lang="jsx" setup>
import {reactive,watch,onMounted} from 'vue'
import {agents} from '@/api/agent'
import {remove,modify} from '@/api/proxy'
import ProxyForm from './form.vue'
import { MessagePlugin } from 'tdesign-vue-next';
// import './index.scss'

const columns=[
{ colKey: 'bind_port', title: '绑定端口', width: '90' },
{ colKey: 'agent', title: '所属代理端' },
{ colKey: 'target_host', title: '目标主机', width: '140' },
{ colKey: 'target_port', title: '目标端口', width: '90' },
{
    title: '是否启用',
    width:'90',
    colKey: 'enable',
    cell: (h, { row }) => (
      row.enable?<t-popconfirm content="确认启用吗" theme="success" onConfirm={()=>{modifyEnable(row,false)}}>
          <t-tag theme="success" style="cursor:pointer">已启用</t-tag>
      </t-popconfirm>:<t-popconfirm content="确认启用吗" theme="danger" onConfirm={()=>{modifyEnable(row,true)}}>
        <t-tag theme="danger" style="cursor:pointer">已禁用</t-tag>
      </t-popconfirm>
    ),
},
{
    title: '连接白名单',
    colKey: 'whitelist',
    cell: (h, { row }) => (
      <span>{row.whitelist.join(',')}</span>
    ),
},
{
    title: '操作',
    width:'150',
    colKey: 'operation',
    cell: (h, { row }) => (
      <t-popconfirm content="确认移除吗" theme="danger" onConfirm={()=>{removeProxy(row)}}>
          <t-button theme="danger">移除</t-button>
      </t-popconfirm>
    ),
  },
];

const state=reactive({
  data:[],
  visible:false,
})

watch(()=>state.visible,()=>{
  loadData();
})

const loadData=()=>{
  agents().then(res=>{
    const proxys=[];
    for(let i=0;i<res.data.length;i++){
      let p=res.data[i].proxy;
      for(let j=0;j<p.length;j++){
        proxys.push({...p[j],agent:res.data[i].name,agentId:res.data[i].id})
      }
    }
    state.data=proxys;
  })
}

const removeProxy=(row)=>{
  // console.log(row)
  remove(row.bind_port).then(res=>{
    MessagePlugin.success(res.info);
    loadData();
  })
}

const modifyEnable=(row,enable)=>{
  modify({...row,enable}).then(res=>{
    MessagePlugin.success(res.info);
    row.enable=enable;
    // loadData();
  })
}

onMounted(()=>{
  loadData();
})
</script>