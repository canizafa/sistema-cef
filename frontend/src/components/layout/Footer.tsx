export function Footer() {
  return (
    <footer className="bg-primary text-white">

      {/* NEWSLETTER */}
      <section className="bg-destructive px-4 py-8 md:px-12">
        <div className="mx-auto flex max-w-7xl flex-col gap-6 md:flex-row md:items-center md:justify-between">

          {/* TEXTO */}
          <div className="space-y-2">
            <h2 className="text-2xl font-bold">
              Suscríbete a nuestro Newsletter
            </h2>

            <p className="text-base">
              Recibe tips, promociones y novedades
            </p>
          </div>

          {/* FORM */}
          <form className="flex w-full flex-col gap-3 md:w-auto md:flex-row">
            <input
              type="email"
              placeholder="Tu email"
              className="
                h-10
                rounded-md
                border
                border-border
                bg-background
                px-4
                text-primary
                outline-none
                placeholder:text-muted
                md:w-72
              "
            />

            <button
              type="submit"
              className="
                h-10
                rounded-md
                bg-primary
                px-4
                text-sm
                font-medium
                transition
                hover:opacity-90
              "
            >
              Suscribirse
            </button>
          </form>
        </div>
      </section>

      {/* CONTENIDO */}
   <section className="bg-[#222222] px-4 py-12 md:px-12">
  <div className="mx-auto grid max-w-7xl grid-cols-1 gap-8 text-center md:grid-cols-2">

    {/* CONTACTO */}
    <div className="flex flex-col items-center space-y-4">
      <h3 className="text-xl font-semibold">
        Contacto
      </h3>

      <div className="flex flex-col items-center gap-4 text-muted">
        <p>info@cef.com</p>
      </div>
    </div>

    {/* HORARIOS */}
    <div className="flex flex-col items-center space-y-4">
      <h3 className="text-xl font-semibold">
        Horarios
      </h3>

      <div className="space-y-3 text-center text-muted">
        <p>Lunes - Viernes: 6:00 - 22:00</p>
        <p>Sábados: 8:00 - 20:00</p>
        <p>Domingos: 9:00 - 14:00</p>
      </div>
    </div>

  </div>
        {/* BOTTOM */}
        <div
          className="
            mt-10
            flex
            flex-col
            gap-4
            border-t
            border-border
            pt-6
            md:flex-row
            md:items-center
            md:justify-between
          "
        >
          <p className="text-sm mx-auto text-muted">
            © 2026 CEF Centro de Actividades. Todos los derechos reservados.
          </p>

       
        </div>
      </section>
    </footer>
  )
}