/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
        colors: {
            chess: {
                white: '#EEEED2',
                green: '#769656',
                piece: {
                    white: '#FFFFFF',
                    black: '#363636',
                    outline: '#333333',
                }
            },
            page: {
                bar: '#272522',
                background: "#312E2B",
                dark: "#21201D",
                text: '#DFDFDE',
                textinverse: '#202021',
            }

        },
        maxWidth: {
            'piece': '128px',
        },
        maxHeight: {
            'piece': '128px',
        },
        boxShadow: {
            'square-inner': 'inset 0 0 0 4px rgb(0 0 0 / 0.1)',
        },

    },
  },
  plugins: [],
}