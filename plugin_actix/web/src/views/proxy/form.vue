<template>
  <t-dialog
      v-model:visible="props.modelValue"
      :header="props.title"
      mode="modal"
      draggable
      :on-close="onClose"
      :on-confirm="onConfirm"
    >
      <template #body>
        <t-form>
          <t-form-item label="代理端ID" name="agent_id">
            <t-input placeholder="请输入内容" v-model="state.formData.agent_id"/>
          </t-form-item>
          <t-form-item label="绑定端口" name="bind_port">
            <t-input-number placeholder="请输入内容" theme="normal" :max="65536" :min="4000"  v-model="state.formData.bind_port"/>
          </t-form-item>
          <t-form-item label="目标主机" name="target_host">
            <t-input placeholder="请输入内容"  v-model="state.formData.target_host"/>
          </t-form-item>
          <t-form-item label="目标端口" name="target_port">
            <t-input-number placeholder="请输入内容" theme="normal" :max="65536" :min="1"  v-model="state.formData.target_port"/>
          </t-form-item>
          <t-form-item label="白名单" name="whitelist">
            <t-textarea placeholder="请输入内容"  v-model="state.formData.whitelist" :autosize="{ minRows: 5, maxRows: 5 }"/>
          </t-form-item>
        </t-form>
      </template>
    </t-dialog>
</template>
<script setup>
import {reactive,watch,defineProps,defineEmits} from 'vue'
import {add} from '@/api/proxy'
import { MessagePlugin } from 'tdesign-vue-next';

const emit=defineEmits(['update:modelValue'])

const props=defineProps({
  title:{
    type:String,
    default:'新增代理端'
  },
  modelValue:{
    type:Boolean,
    default:false
  }
})

const state=reactive({
  visibleModal:false,
  formData:{
    agent_id:'',
    bind_port:20000,
    target_host:'127.0.0.1',
    target_port:8080,
    enable:false,
    whitelist:''
  }
})


const onConfirm=()=>{
  
  add(state.formData.agent_id,{...state.formData,whitelist:state.formData.whitelist.split('\n')}).then(res=>{
    emit('update:modelValue',false);
    MessagePlugin.success(res.info);
  })
}
const onClose=()=>{
  emit('update:modelValue',false);
}
</script>