<template>
  <div class="login-container">
    <div class="login-panel">
      <div class="panel-left">
        <span>EAST</span>
      </div>
      <div class="panel-right">
        <t-form ref="form" :data="formData" :colon="true" :label-width="0" @reset="onReset" @submit="onSubmit">
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
    });
    
  } else {
    console.log('Validate Errors: ', firstError, validateResult);
    MessagePlugin.warning(firstError);
  }
};

</script>