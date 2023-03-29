<template>
  <div class="login-container">
    <div class="login-panel">
      <div class="panel-left">
        <svg xmlns="http://www.w3.org/2000/svg" version="1.1" height="60" width="60">
            <circle cx="30" cy="30" r="20"  fill="#ef9621" >
              <animate attributeName="cy" from="90" to="50" dur="1s"  />
          </circle>
          <rect width="200" height="20" y="50"
          style="fill:#25abf0;"/>
          </svg> 
        <span class="title">East</span>
        
      </div>
      <div class="panel-right">
        <t-form ref="form" :rules="rules" :data="formData" :colon="true" :label-width="0" @reset="onReset" @submit="onSubmit">
          <t-form-item name="username">
            <t-input v-model="formData.username" clearable placeholder="请输入用户名">
              <template #prefix-icon>
                <desktop-icon />
              </template>
            </t-input>
          </t-form-item>

          <t-form-item name="password">
            <t-input v-model="formData.password" type="password" clearable placeholder="请输入密码">
              <template #prefix-icon>
                <lock-on-icon />
              </template>
            </t-input>
          </t-form-item>

          <t-form-item>
            <t-button theme="primary" type="submit" block>登录</t-button>
          </t-form-item>
        </t-form>
      </div>
    </div>
  </div>
</template>
<script setup>
import {reactive} from 'vue'
import { MessagePlugin } from 'tdesign-vue-next';
import { DesktopIcon, LockOnIcon } from 'tdesign-icons-vue-next'
import './index.scss'
import {login} from '@/api/user'
import {useRouter} from 'vue-router'

const router=useRouter();

const formData=reactive({
    username:'',
    password:'',
})

const rules = {
  username: [
    { required: true, message: '用户名必填', type: 'error', trigger: 'blur' },
    { required: true, message: '用户名必填', type: 'error', trigger: 'change' },
    { whitespace: true, message: '用户名不能为空' },
    { min: 3, message: '输入用户名字符应在3到12之间', type: 'error', trigger: 'blur' },
    { max: 16, message: '输入用户名字符应在3到16之间', type: 'error', trigger: 'blur' },
  ],
  password: [{ required: true, message: '密码必填', type: 'error' }],
};

const onReset = () => {
  formData.username='';
  formData.password='';
  MessagePlugin.success('重置成功');
};

const onSubmit = ({ validateResult, firstError }) => {
  if (validateResult === true) {
    login(formData.username.trim(),formData.password.trim()).then(res=>{
      console.log(res);
      localStorage.setItem('east-token',res.data);
      MessagePlugin.success('登录成功');
      router.push({path:'/'})
    }).catch(e=>{
      formData.password='';
    });
    
  } else {
    console.log('Validate Errors: ', firstError, validateResult);
    MessagePlugin.warning(firstError);
  }
};

</script>