#set text(font: "Times New Roman", size: 14pt)
#set page(
  paper: "a4",
  margin: (top: 1.5cm, bottom: 1.5cm, left: 2.5cm, right: 1.5cm)
)
#align(center)[
#image("kpi.png", width: 75%)

Міністерство освіти і науки України

Національний технічний університет України

“Київський політехнічний інститут імені Ігоря Сікорського”

Факультет інформатики та обчислювальної техніки

Кафедра інформатики та програмної інженерії

#align(horizon)[
  #text(size: 18pt)[
    *Лабораторна робота №3*

    Методологія інженерії програмного забезпечення
    ]

    Тема: Вибіркові дослідження (Sample Studies)
  ]

  #columns(2, gutter: 8pt)[
    #align(left)[
      Виконав:

      студент групи ІП-51мн

      Панченко С. В.
    ]

    #colbreak()

    #align(right)[
      Перевірив:

      Щебланін Ю. М.
    ]
  ]
]

#align(center + bottom)[
  Київ 2026
]

#set page(numbering: "1")
#show outline: it => {
  show heading: set align(center)
  show heading: set text(weight: "regular")
  it
}
#outline(title: upper([Зміст]))

#set heading(numbering: (..nums) => nums.pos().map(str).join("."))
#show heading: it => {
  if it.level == 1 {
    counter(figure.where(kind: image)).update(0)
    set align(center)
    set text(weight: "regular", size: 18pt)
    pagebreak()
    upper(it)
  } else {
    set text(weight: "regular", size: 14pt)
    it
  }
}
#show figure: it => {
  set align(center)
  it.body
  v(8pt, weak: true)
  it.supplement 
  [ ]
  context (it.counter.display(it.numbering))
  [ — ] 
  it.caption.body
}
#set figure(
  supplement: [Рисунок],
  numbering: (num) => {
    context {
      let h_num = counter(heading).at(here()).at(0)
      str(h_num) + "." + str(num)
    }
  }
)

#show ref: it => {
  let el = it.element
  if el != none and el.func() == figure {
    context {
      let num = el.counter.at(el.location())
      numbering(el.numbering, ..num)
    }
  } else {
    it
  }
}
#set par(first-line-indent: (amount: 1.25cm, all: true), justify: true, leading: 1em, spacing: 1em)
#set list(indent: 1.25cm)
#set enum(indent: 1.25cm)

= Мета
Ознайомитися з типами досліджень, які виконуються в будь-яких умовах, зокрема з вибірковими дослідженнями системних артефактів (Sample Studies) @russo2019. Досягти статистичного узагальнення соціально-комунікаційних патернів розробників відкритого програмного забезпечення.

= Завдання

+ Визначити мету дослідження та обрати метод дослідження (Sample Study). Погодити з викладачем.
+ Створити план дослідження та обрати або створити необхідні інструменти дослідження (GraphQL API, Rust).
+ Виконати дослідження (зібрати вибірку даних).
+ Аналізувати дані дослідження та зробити висновки щодо репрезентативності та узагальнення.
+ Створити звіт за результатами дослідження.

= Виконання

Генеральною популяцією для дослідження є всі публічні проєкти з відкритим вихідним кодом на платформі GitHub. Оскільки проаналізувати всі існуючі системні артефакти неможливо, було сформовано репрезентативну вибірку (sample) із 28 найбільш активних репозиторіїв, що охоплюють різні мови програмування (Rust, Go, Python), веб-фреймворки (React, Vue, Django), інфраструктурні рішення (Kubernetes, Terraform) та платформи машинного навчання (TensorFlow, PyTorch).

Для збору даних (Data Collection) було використано GitHub GraphQL API @github_api. За допомогою розробленого програмного забезпечення на мові Rust @rust_lang було завантажено та проаналізовано метадані понад 140 000 Pull Requests (до 5000 найновіших артефактів з кожного репозиторію). Такий підхід гарантує високу статистичну значущість і дозволяє узагальнити (generalize) результати на всю індустрію.

== Перевірка універсальності «Закону тривіальності»

На рисунках @1_size_vs_density_unfiltered та @2_size_vs_density_filtered зображено агреговану крос-проєктну залежність між обсягом внесених змін (розміром PR) та щільністю обговорення (кількість коментарів на рядок коду). 

#figure(
  image("build/plots/1_size_vs_density_unfiltered.png", width: 90%),
  caption: [Залежність розміру PR від щільності коментарів (крос-проєктна вибірка)]
) <1_size_vs_density_unfiltered>

#figure(
  image("build/plots/2_size_vs_density_filtered.png", width: 90%),
  caption: [Залежність розміру PR від щільності коментарів (фільтрація до 1000 рядків)]
) <2_size_vs_density_filtered>

Агреговані дані наочно доводять, що "Закон тривіальності" (Bikeshedding) @parkinson1957 є універсальним психологічним патерном, а не специфікою окремої спільноти. У масштабах усієї вибірки зберігається класична L-подібна залежність: мікро-зміни провокують найбільші обговорення, тоді як масштабні архітектурні зміни проходять без належного аналізу через когнітивне перевантаження рев'юерів.

Рисунок @3_files_vs_comments підтверджує цю гіпотезу на рівні архітектурної складності: при зміні понад 40-50 файлів сумарна кількість коментарів різко падає, незалежно від технологічного стеку проєкту.

#figure(
  image("build/plots/3_files_vs_comments.png", width: 90%),
  caption: [Кореляція кількості змінених файлів та коментарів у вибірці]
) <3_files_vs_comments>

== Документація як глобальний «комунікаційний запобіжник»

Наступним кроком було перевірено, чи впливає якість текстового опису артефакту на обсяг наступних дискусій у масштабах різних екосистем. Результати крос-проєктного аналізу наведено на рисунках @4_body_len_vs_comment_count та @5_body_len_vs_total_comment_len.

#figure(
  image("build/plots/4_body_len_vs_comment_count.png", width: 90%),
  caption: [Залежність кількості коментарів від довжини опису PR (Sample Study)]
) <4_body_len_vs_comment_count>

#figure(
  image("build/plots/5_body_len_vs_total_comment_len.png", width: 90%),
  caption: [Залежність загального обсягу дискусії від довжини опису PR]
) <5_body_len_vs_total_comment_len>

Вибіркове дослідження повністю підтвердило попередні локальні спостереження. Відсутність детального опису (менше 500 символів) стабільно корелює з комунікаційними аномаліями. Розширений контекст (понад 1500 символів) слугує надійним запобіжником від нескінченних уточнень у будь-якому open-source проєкті.

== Системний прояв синдрому «Пінг-Понгу»

На рисунку @6_ping_pong_syndrome відображено агреговану динаміку ітеративного рев'ю (чергування коментарів та дрібних комітів) для всіх 140 000 проаналізованих Пул-реквестів.

#figure(
  image("build/plots/6_ping_pong_syndrome.png"),
  caption: [Синдром Пінг-Понгу: крос-проєктна динаміка часу до злиття]
) <6_ping_pong_syndrome>

Дані репрезентативної вибірки свідчать, що деструктивний вплив мікро-ітерацій є фундаментальною проблемою інженерії ПЗ @bacchelli2013. Зростання кількості циклів «Пінг-Понгу» експоненціально збільшує час до інтеграції коду (Time-to-Merge). Незалежно від інфраструктури (чи то проєкт Facebook/React, чи Linux Kernel), 5-7 циклів обговорень збільшують час доставки функціоналу на порядок (від 1-2 днів до місяця).

= Висновок

У ході лабораторної роботи було успішно проведено вибіркове дослідження (Sample Study) системних програмних артефактів. Шляхом вилучення вибірки обсягом понад 140 000 Pull Requests із 28 різних репозиторіїв відкритого коду було досягнуто високого рівня узагальненості (generalizability) результатів.

Доведено, що такі явища, як "Закон тривіальності" (уникнення глибокого рев'ю складного коду), зниження комунікаційного навантаження завдяки якісній документації, та деструктивний вплив синдрому "Пінг-Понгу" не є локальними проблемами окремих спільнот. Вони є фундаментальними поведінковими та соціально-комунікаційними патернами усієї генеральної популяції інженерів програмного забезпечення. 

Отримані емпіричні дані підкреслюють необхідність стандартизації процесів Code Review та впровадження жорстких політик щодо наявності описів коду у промисловій розробці.

// ТУТ МАГІЯ TYPST ДЛЯ АВТОМАТИЧНОЇ БІБЛІОГРАФІЇ
#bibliography("refs.bib", title: "Перелік посилань", style: "ieee")

#counter(heading).update(0)
#let ukr_alphabet = ("А", "Б", "В", "Г", "Д", "Е", "Є", "Ж", "З", "И", "І", "Ї", "Й", "К", "Л", "М", "Н", "О", "П", "Р", "С", "Т", "У", "Ф", "Х", "Ц", "Ч", "Ш", "Щ", "Ю", "Я")
  
#set heading(numbering: (..nums) => {
  let values = nums.pos()
  let n = values.at(0)
  
  let letter = if n <= ukr_alphabet.len() {
    ukr_alphabet.at(n - 1)
  } else {
    str(n)
  }
  
  if values.len() == 1 {
    letter
  } else {
    letter + "." + values.slice(1).map(str).join(".")
  }
})

#show heading: it => {
  if it.level == 1 {
    counter(figure.where(kind: image)).update(0)
    
    set align(center)
    set text(weight: "regular", size: 18pt)
    pagebreak()
    
    block[
      Додаток #counter(heading).display() \
      #text(it.body)
    ]
  } else {
    set text(weight: "regular", size: 14pt)
    it
  }
}

= Лістинг коду

#let embed_code(file_path, language) = {
  heading(file_path, level: 2)
  raw(read(file_path), lang: language, block: true)
}

#embed_code("graphql/pr_query.graphql", "graphql")
#embed_code("src/bin/fetch.rs", "rust")
#embed_code("src/bin/stats.rs", "rust")
#embed_code("src/bin/plotters.rs", "rust")