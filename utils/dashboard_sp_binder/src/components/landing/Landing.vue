<template>
  <div style="max-height:480px;">
    <section class="hero is-light">
      <div class="hero-body">
        <div class="container">
          <h1 class="title" style="margin-top:-39px;">
          <span>SkyPirl Account</span>
	  </h1>
	  <p class="small_text">Send 5 Pirl in "Transfer" menu to activate the Machine, or switch on Cinema.
	 </p><br>
	<p>To restore account from JSON file or create a new account, use "Accounts" menu.</p><br>

        </div>
      </div>
    </section>
  </div>
</template>
<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';

@Component({})
export default class Landing extends Vue {

  public async mounted() {
    this.$store.commit('setExplorer',{provider: 'subscan', chain: 'kusama'})
  }
}
</script>

<style scoped>
.subtitle {
  text-decoration: underline;
}
</style>
