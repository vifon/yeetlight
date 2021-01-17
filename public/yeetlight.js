'use strict';

axios.get("/config.json").then(res => {
  const config = res.data
  const initialState = {
    bulbs: {}
  }
  for (name in config.bulbs) {
    const bulb = config.bulbs[name]
    initialState.bulbs[name] = {
      name: name,
      addr: bulb.addr || name,
      linked: bulb.linked || [],
      power: undefined,
      brightness: undefined,
      temperature: undefined
    }
  }

  const store = new Vuex.Store({
    state: initialState,
    mutations: {
      power(state, { bulb, power }) {
        state.bulbs[bulb].power = power
      },
      brightness(state, { bulb, brightness }) {
        state.bulbs[bulb].brightness = brightness
      },
      temperature(state, { bulb, temperature }) {
        state.bulbs[bulb].temperature = temperature
      }
    },
    actions: {
      setPower(context, { bulb, power }) {
        switch (power) {
        case true:
        case false:
          const addr = context.getters.addr(bulb)
          return axios.post(
            "/" + (power ? "on" : "off") + "?bulb=" + addr
          ).then(res => {
            context.commit('power', { bulb, power })
          })
        default:
          return context.commit('power', { bulb, power: undefined })
        }
      },
      setBrightness(context, { bulb, brightness }) {
        if (context.getters.power(bulb) !== true) {
          context.dispatch('setPower', { bulb, power: true })
        }
        const addr = context.getters.addr(bulb)
        return axios.post(
          "/brightness?bulb=" + addr + "&brightness=" + brightness
        ).then(() => {
          context.commit('brightness', { bulb, brightness })
        })
      },
      setTemperature(context, { bulb, temperature }) {
        if (context.getters.power(bulb) !== true) {
          context.dispatch('setPower', { bulb, power: true })
        }
        const addr = context.getters.addr(bulb)
        return axios.post(
          "/temperature?bulb=" + addr + "&temperature=" + temperature
        ).then(() => {
          context.commit('temperature', { bulb, temperature })
        })
      }
    },
    getters: {
      addr: state => bulb => {
        return state.bulbs[bulb].addr
      },
      power: state => bulb => {
        return state.bulbs[bulb].power
      },
      brightness: state => bulb => {
        return state.bulbs[bulb].brightness
      },
      temperature: state => bulb => {
        return state.bulbs[bulb].temperature
      }
    }
  })

  Vue.component('bulb', {
    props: ['addr', 'name'],
    template: "#bulb-template",
    data() {
      return {
        linked: [],

        /* Temporary local values for deferred application. */
        localBrightness: undefined,
        localTemperature: undefined
      }
    },
    methods: {
      setPower(newValue) {
        this.$store.dispatch('setPower', { bulb: this.name, power: newValue })
        this.linked.filter(
          link => link.enable
        ).forEach(link => {
          this.$store.dispatch(
            'setPower', { bulb: link.name, power: newValue }
          )
        })
      },
      setBrightness(newValue) {
        this.$store.dispatch(
          'setBrightness', { bulb: this.name, brightness: newValue }
        ).then(() => {
          this.localBrightness = undefined
        })

        this.linked.filter(
          link => link.enable
        ).forEach(link => {
          this.$store.dispatch(
            'setBrightness', { bulb: link.name, brightness: newValue }
          )
        })
      },
      setTemperature(newValue) {
        this.$store.dispatch(
          'setTemperature', { bulb: this.name, temperature: newValue }
        ).then(() => {
          this.localTemperature = undefined
        })

        this.linked.filter(
          link => link.enable
        ).forEach(link => {
          this.$store.dispatch(
            'setTemperature', { bulb: link.name, temperature: newValue }
          )
        })
      }
    },
    computed: {
      power() {
        return this.$store.getters.power(this.name)
      },
      brightness: {
        get() {
          if (this.localBrightness === undefined) {
            return this.$store.getters.brightness(this.name)
          } else {
            return this.localBrightness
          }
        },
        set(newValue) {
          this.localBrightness = newValue
        }
      },
      temperature: {
        get() {
          if (this.localTemperature === undefined) {
            return this.$store.getters.temperature(this.name)
          } else {
            return this.localTemperature
          }
        },
        set(newValue) {
          this.localTemperature = newValue
        }
      }
    },
    mounted() {
      axios.get("/info?bulb=" + this.addr).then(res => {
        const info = res.data
        this.$store.commit('brightness', {
          bulb: this.name,
          brightness: info.bright
        })
        this.$store.commit('temperature', {
          bulb: this.name,
          temperature: info.ct
        })
        this.$store.commit('power', {
          bulb: this.name,
          power: info.power === "on"
        })

        this.linked = this.$store.state.bulbs[this.name].linked.map(link => ({
          name: link,
          enable: false
        }))
      })
    }
  });

  var app = new Vue({
    el: '#app',
    store: store
  })
})
