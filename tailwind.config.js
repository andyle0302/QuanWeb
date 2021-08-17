module.exports = {
  purge: [
    './templates/**/*.jinja',
  ],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            p: {
              marginTop: '0.75em',
              marginBottom: '0.75em',
            },
            pre: {
              marginTop: '1em',
              marginBottom: '1em',
              lineHeight: 1.5,
              fontSize: '0.75em',
            },
            img: {
              marginTop: '1em',
              marginBottom: '1em',
            },
            a: {
              '&:hover': {
                color: '#2c5282',
              },
            },
            hr: {
              marginTop: '1em',
              marginBottom: '1em',
            }
          }
        }
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/aspect-ratio'),
    require('@tailwindcss/typography'),
  ],
}
