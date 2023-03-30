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
          <t-form-item label="ID" name="id">
            <t-input placeholder="请输入内容" v-model="state.formData.id"/>
          </t-form-item>
          <t-form-item label="代理端" name="name">
            <t-input placeholder="请输入内容"  v-model="state.formData.name"/>
          </t-form-item>
        </t-form>
      </template>
    </t-dialog>
</template>
<script setup>
import {reactive,watch,defineProps,defineEmits} from 'vue'
import {add} from '@/api/agent'
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
    id:'',
    name:'',
  }
})


const onConfirm=()=>{
  
  add(state.formData).then(res=>{
    emit('update:modelValue',false);
    MessagePlugin.success(res.info);
  })
}
const onClose=()=>{
  emit('update:modelValue',false);
}
</script>