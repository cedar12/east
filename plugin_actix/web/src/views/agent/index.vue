<template>
  <div class="agent-container">
    <t-space direction="vertical">
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
    </t-space>
      <AgentForm v-model="state.visible"/>
    </div>
</template>
<script lang="jsx" setup>
import {reactive,watch,onMounted} from 'vue'
import {agents,remove} from '@/api/agent'
import AgentForm from './form.vue'
import { MessagePlugin } from 'tdesign-vue-next';
import './index.scss'

const columns=[
{ colKey: 'id', title: 'ID', width: '150' },
{ colKey: 'name', title: '代理端' },
{
    title: '是否在线',
    width:'150',
    colKey: 'operation',
    cell: (h, { row }) => (
      row.is_online?<t-tag theme="success">在线</t-tag>:<t-tag theme="danger">离线</t-tag>
    ),
  },
{
    title: '操作',
    width:'150',
    colKey: 'operation',
    cell: (h, { row }) => (
      <t-popconfirm content="确认删除吗" theme="danger" onConfirm={()=>{removeAgent(row)}}>
          <t-button theme="danger">移除</t-button>
      </t-popconfirm>
    ),
  },
];

const state=reactive({
  data:[],
  visible:false,
})

watch(()=>state.visible,(n)=>{
  if(n===false)loadData();
})

const loadData=()=>{
  agents().then(res=>{
    state.data=res.data;
  })
}

const removeAgent=(row)=>{
  // console.log(row)
  remove(row.id).then(res=>{
    MessagePlugin.success(res.info);
    loadData();
  })
}

onMounted(()=>{
  loadData();
})
</script>