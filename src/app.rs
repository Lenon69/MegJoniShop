use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="pl">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="keywords" content="odzież używana, second hand online, sklep vintage, ciuchy z drugiej ręki, moda ekologiczna, ubrania używane, outlet, odzież damska używana, odzież męska używana" />
                <meta name="author" content="Meg Joni" />
                <meta name="robots" content="index, follow" />
                <meta name="theme-color" content="#ffffff" />
                <meta
                  property="og:title"
                  content="Meg Joni - Odkryj Unikalną Odzież Używaną Online"
                />
                <meta
                  property="og:description"
                  content="Znajdź stylowe perełki z drugiej ręki i odśwież swoją garderobę w ekologiczny sposób. Wysoka jakość w świetnych cenach!"
                />
                <meta property="og:type" content="website" />
                <meta property="og:url" content="https://www.megjoni.pl/" />
                <meta property="og:site_name" content="Meg Joni" />
                <meta property="og:locale" content="pl_PL" />

                <link rel="canonical" href="https://www.megjoni.pl/" />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style.css"/>
        <Title text="Meg Joni - Odzież Używana Online | Najlepsze Second Hand Odkrycia"/>

        <Router>
            <Header />
            <Navbar />

            <main>
                <Routes fallback=|| "Strona nie istnieje.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("woman") view=WomanPage/>
                    <Route path=StaticSegment("man") view=MenPage/>
                    <Route path=StaticSegment("new-arrivals") view=NewsPage/>
                    <Route path=StaticSegment("sale") view=SalePage/>
                    <Route path=StaticSegment("about") view=AboutPage/>
                    <Route path=StaticSegment("contact") view=ContactPage/>

                    <Route path=StaticSegment("privacy") view=PrivacyPage/>
                    <Route path=StaticSegment("shipping") view=ShippingReturnsPage/>
                    <Route path=StaticSegment("terms") view=TermsAndConditionsPage/>
                </Routes>
            </main>

            <Footer />
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
      <header>
        <div class="logo-title">
          <a href="/">
            <img
              src="/megjoni-big.png"
              alt="Nazwa Sklepu Meg Joni / Logo"
              height="100%"
            />
          </a>
        </div>

        <div class="search-bar">
          <form action="/search" method="get">
            <label for="search-input" class="visually-hidden"
              >Szukaj produktów</label
            >
            <input
              type="text"
              id="search-input"
              name="query"
              placeholder="Szukaj..."
            />
            <button type="submit">Szukaj</button>
          </form>
        </div>

        <div class="user-cart-icons">
          <a href="/account" aria-label="Moje konto">
            <img src="/my-account.svg" width="32" height="32" />
          </a>
          <a href="/cart" aria-label="Mój koszyk">
            <img src="/shopping-cart.svg" width="32" height="32" />
            <span class="cart-count">"0"</span>
          </a>
        </div>
    </header>
      }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
    <nav>
        <ul>
        <li><a href="/">Strona Główna</a></li>
        <li><a href="/woman">Damska</a></li>
        <li><a href="/man">Męska</a></li>
        <li><a href="/new-arrivals">Nowości</a></li>
        <li><a href="/sale">Wyprzedaż</a></li>
        <li><a href="/about">O Nas</a></li>
        <li><a href="/contact">Kontakt</a></li>
      </ul>
    </nav>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="hero-section">
            <img src="/clothes1.jpg"
                alt="Odkryj unikalne perełki z drugiej ręki"
                class="hero-image" />
            <div class="hero-text">
                <h2>Moda z Duszą - Znajdź Swoje Unikalne Perełki</h2>
                <p>Wysokiej jakości odzież używana, starannie wyselekcjonowana dla Ciebie.</p>
                <a href="/woman">
                    <button>Przejdź do sklepu</button>
                </a>
            </div>
        </section>

        <section class="featured-products">
            <h2>Polecane produkty</h2>
             <div class="product-grid">
                 <article class="product-item">
                     <a href="product/id-produktu">
                         <figure>
                             <img src=r"/spodnie-vintage.jpg" alt="Opis produktu. Np. Czerwona sukienka" width="300" height="400" />
                         </figure>
                         <h3>Spodnie Vintage</h3>
                         <p class="product-price">49.99 PLN</p>
                     </a>
                 </article>
             </div>
             <div class="view-all-link">
                 <a href="/woman">Zobacz wszystkie produkty</a>
             </div>
        </section>
        <section class="about-promo">
            <h2>Dlaczego Second Hand?</h2>
            <p>
                Moda z drugiej ręki to świadomy wybór - ekologiczny, ekonomiczny i
                niepowtarzalny. Daj ubraniom drugie życie!
            </p>
            <a href="/about" class="btn">
                <button>Dowiedz się więcej o nas</button>
            </a>
        </section>

    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-links">
                <ul>
                    <li><a href="/contact">Kontakt</a></li>
                    <li><a href="/shipping">Wysyłka i zwroty</a></li>
                    <li><a href="/privacy">Polityka Prywatności</a></li>
                    <li><a href="/terms">Regulamin sklepu</a></li>
                </ul>
            </div>

            <div class="social-media">
                <p>Znajdź nas w social mediach:</p>
                <a href="https://www.facebook.com/MegJoni"
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Facebook">
                    <img src="/facebook.svg" width="48" height="48"/>
                </a>
                <a href="https://www.instagram.com/Meg.joni"
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Instagram">
                    <img src="/instagram.svg" width="48" height="48"/>
                </a>
            </div>

            <div class="copyright">
                <p>
                    "©"<span id="current-year">2025</span> Meg Joni. Wszelkie prawa zastrzeżone.
                </p>
            </div>
        </footer>
    }
}

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <main>
            <section class="about-promo"> // Używamy istniejącej klasy z HomePage dla spójności
            <div class="container">
                <h2>O Nas</h2>
                <img src="megjoni-big.png" id="megjoni-logo"/>
            </div>
                <p>
                    Witaj w Meg Joni! Jesteśmy pasjonatami mody z drugiej ręki, wierzymy, że ubrania
                    zasługują na drugie życie. Nasz sklep to miejsce, gdzie znajdziesz unikalne
                    perełki vintage i starannie wyselekcjonowaną odzież używaną w doskonałym stanie.
                </p>
                <p>
                    Naszą misją jest promowanie zrównoważonej mody i pokazywanie, że można ubierać się
                    stylowo, dbając jednocześnie o planetę. Każdy zakup w naszym sklepie to krok
                    w stronę bardziej świadomego konsumpcjonizmu.
                </p>
                <p>
                    Dołącz do naszej społeczności miłośników second handu i odkryj swój niepowtarzalny styl!
                </p>
                 <a href="/woman">
                    <button>Zobacz nasze produkty</button>
                 </a>
            </section>
        </main>
    }
}

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <main>
            <section class="contact-section" style="max-width: 600px; margin: var(--space-md) auto; background-color: var(--color-surface); padding: var(--space-md); border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);"> // Proste style inline dla przykładu, możesz przenieść do CSS
                <h2>Skontaktuj się z Nami</h2>
                <p>
                    Masz pytania dotyczące produktów, zamówień, czy współpracy? Chętnie pomożemy!
                    Skontaktuj się z nami poprzez formularz poniżej lub bezpośrednio.
                </p>

                <div class="contact-info" style="margin-bottom: var(--space-md);">
                    <p><strong>Email: </strong> <a href="mailto:kontakt@megjoni.pl">kontakt@megjoni.pl</a></p>
                    // Dodaj opcjonalnie telefon, adres itp.
                </div>

                <form action="/submit-contact-form" method="post"> // To jest tylko placeholder, rzeczywista obsługa wymaga backendu
                    <div style="margin-bottom: var(--space-sm);">
                        <label for="name" class="visually-hidden">Twoje imię:</label>
                        <input type="text" id="name" name="name" placeholder="Twoje imię" required style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;" />
                    </div>
                    <div style="margin-bottom: var(--space-sm);">
                        <label for="email" class="visually-hidden">Twój email:</label>
                        <input type="email" id="email" name="email" placeholder="Twój email" required style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;" />
                    </div>
                     <div style="margin-bottom: var(--space-sm);">
                        <label for="subject" class="visually-hidden">Temat:</label>
                        <input type="text" id="subject" name="subject" placeholder="Temat wiadomości" style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;" />
                    </div>
                    <div style="margin-bottom: var(--space-sm);">
                        <label for="message" class="visually-hidden">Twoja wiadomość:</label>
                        <textarea id="message" name="message" placeholder="Twoja wiadomość" rows="6" required style="width: 100%; padding: var(--space-xs); border: 1px solid var(--color-border); border-radius: 4px;"></textarea>
                    </div>
                    <button type="submit">Wyślij wiadomość</button>
                </form>
            </section>
        </main>
    }
}

#[component]
pub fn WomanPage() -> impl IntoView {
    view! {
        <main>
            <section>
                <h2>Kategoria: Damska</h2>
                <p>"Odkryj naszą kolekcję odzieży damskiej z drugiej ręki. Eleganckie sukienki, wygodne spodnie, stylowe bluzki i wiele więcej!"</p>

                // Placeholder dla siatki produktów damskich
                <div class="product-grid">
                     // Skopiuj i wklej przykładowe product-itemy z HomePage lub stwórz nowe
                     // W rzeczywistości tutaj będzie pętla iterująca po danych produktów
                    <article class="product-item">
                        <a href="/product/damskie-001">
                            <figure>
                                <img src="/czerwona-sukienka.jpg" alt="Przykładowy produkt damski" width="300" height="400" />
                            </figure>
                            <h3>Czerwona sukienka</h3>
                            <p class="product-price">75.00 PLN</p>
                        </a>
                    </article>
                     <article class="product-item">
                        <a href="/product/damskie-002">
                            <figure>
                                <img src="/elegancka-sukienka.jpg" alt="Przykładowy produkt damski" width="300" height="400" />
                            </figure>
                            <h3>Elegancka sukienka</h3>
                            <p class="product-price">55.00 PLN</p>
                        </a>
                    </article>
                    // Dodaj więcej placeholderów produktów damskich
                </div>
            </section>
        </main>
    }
}

#[component]
pub fn MenPage() -> impl IntoView {
    view! {
        <main>
            <section>
                <h2>Kategoria: Męska</h2>
                <p>"Przeglądaj naszą ofertę męskiej odzieży używanej. Znajdź koszule, spodnie, marynarki i inne elementy garderoby w świetnych cenach."</p>

                 // Placeholder dla siatki produktów męskich
                <div class="product-grid">
                     // Skopiuj i wklej przykładowe product-itemy z HomePage lub stwórz nowe
                     // W rzeczywistości tutaj będzie pętla iterująca po danych produktów
                    <article class="product-item">
                        <a href="/product/meskie-001">
                            <figure>
                                <img src="/black-tshirt.jpg" alt="Czarny T-Shirt Męski" width="300" height="400" />
                            </figure>
                            <h3>Czarny T-Shirt Męski</h3>
                            <p class="product-price">39.50 PLN</p>
                        </a>
                    </article>
                     <article class="product-item">
                        <a href="/product/meskie-002">
                            <figure>
                                <img src="/niebieska-bluza.jpg" alt="Przykładowy produkt męski" width="300" height="400" />
                            </figure>
                            <h3>Niebieska Bluza</h3>
                            <p class="product-price">85.00 PLN</p>
                        </a>
                    </article>
                    // Dodaj więcej placeholderów produktów męskich
                </div>
            </section>
        </main>
    }
}

#[component]
pub fn NewsPage() -> impl IntoView {
    view! {
        <main>
            <section>
                <h2>Nowości u Meg Joni</h2>
                <p>"Zobacz nasze najnowsze dostawy! Świeże i unikalne ubrania dodane do sklepu."</p>

                 // Placeholder dla siatki najnowszych produktów
                <div class="product-grid">
                     // Skopiuj i wklej przykładowe product-itemy z HomePage lub stwórz nowe
                     // Upewnij się, że używasz zdjęć pasujących do "nowości"
                    <article class="product-item">
                        <a href="/product/nowosc-001">
                            <figure>
                                <img src="/bluza-oversize.jpg" alt="Najnowszy produkt" width="300" height="400" />
                            </figure>
                            <h3>Bluza Oversize</h3>
                            <p class="product-price">65.00 PLN</p>
                        </a>
                    </article>
                    // Dodaj więcej placeholderów nowości
                </div>
            </section>
        </main>
    }
}

#[component]
pub fn SalePage() -> impl IntoView {
    view! {
        <main>
            <section>
                <h2>Wyprzedaż</h2>
                <p>"Super okazje czekają! Ostatnie sztuki w niższych cenach."</p>

                // Placeholder dla siatki produktów na wyprzedaży
                <div class="product-grid">
                     // Skopiuj i wklej przykładowe product-itemy z HomePage lub stwórz nowe
                     // Możesz dodać stylizację do pokazania starej/nowej ceny w przyszłości
                    <article class="product-item">
                        <a href="/product/sale-001">
                            <figure>
                                <img src="letnia-sukienka.jpg" alt="Produkt na wyprzedaży" width="300" height="400" />
                            </figure>
                            <h3>Letnia Sukienka</h3>
                            <p class="product-price"><del style="color: var(--color-text-light);">50.00 PLN</del> 30.00 PLN</p> // Przykład ceny wyprzedażowej
                        </a>
                    </article>
                    // Dodaj więcej placeholderów produktów na wyprzedaży
                </div>
            </section>
        </main>
    }
}

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <main class="max-w-3xl mx-auto p-4 text-gray-800 dark:text-gray-200">
            <h1 class="text-3xl font-bold mb-6">"Polityka Prywatności"</h1>
            <p class="text-sm text-gray-500 mb-8">"Data ostatniej aktualizacji: 23 kwietnia 2025 r."</p>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"1. Administrator danych osobowych"</h2>
                <p>"Administratorem Twoich danych osobowych jest Magdalena Kluba, prowadząca działalność pod nazwą \"Meg Joni\". Możesz się z nami skontaktować pod adresem e-mail: kontakt@megjoni.pl."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"2. Jakie dane zbieramy?"</h2>
                <ul class="list-disc list-inside">
                    <li>"imię i nazwisko"</li>
                    <li>"adres dostawy"</li>
                    <li>"adres e-mail"</li>
                    <li>"numer telefonu (opcjonalnie)"</li>
                    <li>"dane do faktury (jeśli dotyczy)"</li>
                    <li>"adres IP oraz dane o aktywności na stronie (cookies – patrz pkt 6)"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"3. Cel i podstawa prawna przetwarzania danych"</h2>
                <ul class="list-disc list-inside">
                    <li>"realizacja zamówień (art. 6 ust. 1 lit. b RODO)"</li>
                    <li>"prowadzenie konta użytkownika (jeśli dotyczy)"</li>
                    <li>"kontakt z klientem (art. 6 ust. 1 lit. f RODO)"</li>
                    <li>"cele księgowe (art. 6 ust. 1 lit. c RODO)"</li>
                    <li>"cele marketingowe za zgodą (art. 6 ust. 1 lit. a RODO)"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"4. Czas przechowywania danych"</h2>
                <p>"Dane przechowujemy do czasu realizacji umowy i przez okres wymagany przepisami prawa. Dane wykorzystywane do celów marketingowych – do momentu cofnięcia zgody."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"5. Udostępnianie danych"</h2>
                <p>"Dane mogą być przekazywane firmom kurierskim, operatorom płatności, biuru księgowemu oraz firmie hostingowej – tylko w zakresie niezbędnym do świadczenia usług."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"6. Pliki cookies"</h2>
                <p>"Używamy cookies do działania strony, analizy ruchu (np. Google Analytics) i personalizacji treści. Możesz zmienić ich ustawienia w przeglądarce."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"7. Twoje prawa"</h2>
                <ul class="list-disc list-inside">
                    <li>"dostęp do danych"</li>
                    <li>"sprostowanie, usunięcie lub ograniczenie przetwarzania"</li>
                    <li>"przenoszenie danych"</li>
                    <li>"sprzeciw wobec przetwarzania"</li>
                    <li>"cofnięcie zgody"</li>
                    <li>"skarga do Prezesa UODO"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"8. Kontakt"</h2>
                <p>
                    "W sprawach związanych z ochroną danych osobowych, skontaktuj się z nami:"<br/>
                    "📧 E-mail: kontakt@megjoni.pl"<br/>
                    "📬 Adres: Siedziba Łódź"
                </p>
            </section>
        </main>
    }
}

#[component]
pub fn ShippingReturnsPage() -> impl IntoView {
    view! {
        <main class="max-w-3xl mx-auto p-4 text-gray-800 dark:text-gray-200">
            <h1 class="text-3xl font-bold mb-6">"Wysyłka i zwroty"</h1>
            <p class="text-sm text-gray-500 mb-8">"Data ostatniej aktualizacji: 23 kwietnia 2025 r."</p>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"1. Koszt i czas wysyłki"</h2>
                <ul class="list-disc list-inside">
                    <li>"Koszt dostawy na terenie Polski: 14,99 zł"</li>
                    <li>"Czas realizacji zamówienia: 1–3 dni robocze"</li>
                    <li>"Czas dostawy: 1–2 dni robocze od momentu nadania"</li>
                    <li>"Darmowa dostawa dla zamówień powyżej [np. 200 zł]"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"2. Formy dostawy"</h2>
                <ul class="list-disc list-inside">
                    <li>"Kurier (np. InPost, DPD, DHL)"</li>
                    <li>"Paczkomaty InPost"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"3. Zwroty i reklamacje"</h2>
                <p>"Zgodnie z prawem konsumenta masz prawo do zwrotu towaru w ciągu 14 dni od jego otrzymania – bez podania przyczyny."</p>
                <ul class="list-disc list-inside mt-2">
                    <li>"Produkt nie może nosić śladów użytkowania i musi być odesłany w oryginalnym stanie"</li>
                    <li>"Zwrotu dokonujesz na własny koszt"</li>
                    <li>"Zwrot środków nastąpi do 14 dni od otrzymania przesyłki"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"4. Jak dokonać zwrotu?"</h2>
                <ol class="list-decimal list-inside">
                    <li>"Wypełnij formularz zwrotu (dostępny w zakładce Zwroty lub dołączony do przesyłki)"</li>
                    <li>"Zapakuj produkt i odeślij na adres:"<br/>"[Adres do zwrotu]"</li>
                    <li>"Po otrzymaniu i sprawdzeniu przesyłki dokonamy zwrotu pieniędzy"</li>
                </ol>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"5. Reklamacje"</h2>
                <p>"Jeśli produkt jest uszkodzony lub niezgodny z opisem, skontaktuj się z nami pod adresem e-mail: kontakt@megjoni.pl. Do reklamacji dołącz zdjęcia oraz numer zamówienia."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"6. Kontakt"</h2>
                <p>
                    "W razie pytań dotyczących wysyłki lub zwrotów:"<br/>
                    "📧 E-mail: kontakt@megjoni.pl"<br/>
                    "📬 Adres: Siedziba Łódź"
                </p>
            </section>
        </main>
    }
}

#[component]
pub fn TermsAndConditionsPage() -> impl IntoView {
    view! {
        <main class="max-w-3xl mx-auto p-4 text-gray-800 dark:text-gray-200">
            <h1 class="text-3xl font-bold mb-6">"Regulamin sklepu"</h1>
            <p class="text-sm text-gray-500 mb-8">"Data ostatniej aktualizacji: 23 kwietnia 2025 r."</p>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"1. Postanowienia ogólne"</h2>
                <p>
                    "Niniejszy regulamin określa zasady korzystania ze sklepu internetowego prowadzonego pod adresem www.megjoni.pl.
                    Sklep prowadzony jest przez [pełna nazwa firmy, adres, NIP, REGON]."
                </p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"2. Składanie zamówień"</h2>
                <ul class="list-disc list-inside">
                    <li>"Zamówienia można składać 24 godziny na dobę, 7 dni w tygodniu"</li>
                    <li>"Złożenie zamówienia oznacza akceptację niniejszego regulaminu"</li>
                    <li>"Po złożeniu zamówienia klient otrzymuje e-mail z potwierdzeniem przyjęcia zamówienia"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"3. Ceny i płatności"</h2>
                <ul class="list-disc list-inside">
                    <li>"Wszystkie ceny podane w sklepie są cenami brutto i zawierają podatek VAT"</li>
                    <li>"Akceptowane formy płatności: przelew bankowy, szybkie płatności online, BLIK"</li>
                    <li>"Zamówienie jest realizowane po zaksięgowaniu płatności"</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"4. Dostawa"</h2>
                <p>"Informacje o kosztach i czasie dostawy znajdują się w zakładce Wysyłka i zwroty."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"5. Zwroty i reklamacje"</h2>
                <p>"Klient ma prawo do zwrotu towaru w ciągu 14 dni bez podania przyczyny. Szczegóły znajdują się w zakładce Wysyłka i zwroty."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"6. Dane osobowe"</h2>
                <p>"Szczegóły dotyczące przetwarzania danych osobowych znajdują się w Polityce Prywatności."</p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"7. Postanowienia końcowe"</h2>
                <ul class="list-disc list-inside">
                    <li>"Sklep zastrzega sobie prawo do zmiany regulaminu"</li>
                    <li>"W sprawach nieuregulowanych mają zastosowanie przepisy prawa polskiego"</li>
                    <li>"Spory będą rozstrzygane przez właściwy sąd powszechny"</li>
                </ul>
            </section>
        </main>
    }
}
