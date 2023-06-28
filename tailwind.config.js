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
                background: "#EEEED2",
                text: '#EEEED2',
            }

        },
        maxWidth: {
            'piece': '84px',
        },
        boxShadow: {
            'square-inner': 'inset 0 0 0 4px rgb(0 0 0 / 0.1)',
        }

    },
  },
  plugins: [],
}