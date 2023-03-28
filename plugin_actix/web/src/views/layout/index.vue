<template>
  <t-layout class="layout-container">
    <t-header>
      <t-head-menu value="item1" height="120px">
        <template #logo>
          <img width="136" class="logo" src="https://www.tencent.com/img/index/menu_logo_hover.png" alt="logo" />
        </template>
        <!-- <t-menu-item value="item1"> 已选内容 </t-menu-item>
        <t-menu-item value="item2"> 菜单内容一 </t-menu-item>
        <t-menu-item value="item3"> 菜单内容二 </t-menu-item>
        <t-menu-item value="item4" :disabled="true"> 菜单内容三 </t-menu-item> -->
        <template #operations>
          <a href="javascript:;"><t-icon class="t-menu__operations-icon" name="search" /></a>
          <a href="javascript:;"><t-icon class="t-menu__operations-icon" name="notification-filled" /></a>
          <a href="javascript:;"><t-icon class="t-menu__operations-icon" name="user-circle" /></a>
        </template>
      </t-head-menu>
    </t-header>
    <t-layout>
      <t-aside style="border-top: 1px solid var(--component-border)" :class="{collapsed:state.collapsed}">
        <div class="menu-conatiner">
          <t-menu theme="light" :value="router.currentRoute.path" style="margin-right: 50px" :collapsed="state.collapsed">
            <t-menu-item :value="route.path" :to="route" :router="router" v-for="route in state.routes" :key="route.path">
              <template #icon>
                <t-icon :name="route.meta.icon||'dashboard'" />
              </template>
              <span>{{route.meta.title}}</span>
            </t-menu-item>
            
            <template #operations>
              <t-button variant="text" shape="square" @click="changeCollapsed">
                <template #icon><t-icon name="view-list" /></template>
              </t-button>
            </template>
          </t-menu>
        </div>
      </t-aside>
      <t-layout>
        <t-content>
          <div class="content-container">
            <router-view/>
          </div>
        </t-content>
        
      </t-layout>
    </t-layout>
  </t-layout>
</template>
<script setup>
import {reactive} from 'vue'
import {useRouter} from 'vue-router'
import './index.scss'

const router=useRouter();

const state=reactive({
  routes:router.getRoutes().filter(r=>r.meta.title),
  collapsed:false,
})



const changeCollapsed=()=>{
  state.collapsed=!state.collapsed;
}
</script>